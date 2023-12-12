// SDF - render fractals in VR
// by Desmond Germans, 2023

use {
    std::{
        result::Result,
        sync::Arc,
        ptr::copy_nonoverlapping,
        fs,
        path::Path,
    },
    crate::*,
};

#[derive(Clone,Copy,Debug)]
#[repr(u32)]
pub enum Type {
    Quad,
    Cube,
    Cylinder,
    Equirect,
    Fisheye,
}

pub const TILER_FLAGS_STEREO: u32 = 0x00000001;

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct Config {
    pub type_: Type,
    pub flags: u32,
    pub tile_size: Vec2<u32>,

    pub tile_count: Vec2<u32>,
    pub current_tile: Vec2<u32>,
    
    pub fovs: [Fov<f32>; 2],
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct Push {
    pub eye: u32,
    pub face: u32,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct Uniforms {
    pub config: Config,
    pub march: March,
    pub render: Render,
}

pub enum Command {
    Execute(Config,March,Render),
    Exit,
}

pub enum State {
    Idle,
    Rendering,
    Exiting,
}

pub struct Tiler {
    _gpu: Arc<Gpu>,
    queue: Arc<Queue>,

    config: Config,
    march: March,
    render: Render,
    uniform_buffer: Arc<UniformBuffer<Uniforms>>,

    pub state: State,

    assemblies: [Vec<u32>; 2],

    tile_images: [Arc<Image2D>; 2],
    _tile_image_views: [Arc<Image2DView>; 2],

    _pipeline_layout: Arc<PipelineLayout>,
    _descriptor_set_layout: Arc<DescriptorSetLayout>,

    _pipeline: Arc<ComputePipeline>,
    _descriptor_sets: [Arc<DescriptorSet>; 2],
    _shader: Arc<ComputeShader>,
    command_buffers: [Arc<CommandBuffer>; 2],

    counter: usize,
}

impl Tiler {

    pub fn new(gpu: &Arc<Gpu>,code: &[u8],config: Config,march: March,render: Render) -> Result<Tiler,String> {

        // get engine thread queue
        let queue = gpu.queue(2)?;

        // create tile image
        let tile_images = [
            gpu.create_empty_image2d(ImageFormat::RGBA8,config.tile_size.into(),1,1,1,ImageUsage::Storage,AccessStyle::Shared)?,
            gpu.create_empty_image2d(ImageFormat::RGBA8,config.tile_size.into(),1,1,1,ImageUsage::Storage,AccessStyle::Shared)?,
        ];

        // create assemblies
        let assemblies = [
            vec![0u32; (config.tile_size.x * config.tile_size.y * config.tile_count.x * config.tile_count.y) as usize],
            vec![0u32; (config.tile_size.x * config.tile_size.y * config.tile_count.x * config.tile_count.y) as usize],
        ];

        // transition image layout
        queue.transition_image2d_layout(&tile_images[0],PipelineStage::ColorAttachmentOutput,PipelineStage::ColorAttachmentOutput,ImageLayout::General,0,1)?;
        queue.transition_image2d_layout(&tile_images[1],PipelineStage::ColorAttachmentOutput,PipelineStage::ColorAttachmentOutput,ImageLayout::General,0,1)?;

        // create image views
        let tile_image_views = [
            tile_images[0].create_view(0,0,1)?,
            tile_images[1].create_view(0,0,1)?,
        ];

        // create pipeline layout
        let descriptor_set_layout = gpu.build_descriptor_set_layout()
            .uniform_buffer()
            .image2dview()
            .build()?;
        let pipeline_layout = gpu.create_pipeline_layout(&[&descriptor_set_layout],0)?;

        // create uniform buffer
        let uniforms = Uniforms {
            config,
            march,
            render,
        };
        let uniform_buffer = gpu.create_uniform_buffer(&queue,AccessStyle::Shared,&[uniforms])?;

        // create descriptor sets
        let descriptor_sets = [
            descriptor_set_layout.build_descriptor_set()?
                .uniform_buffer(&uniform_buffer)
                .image2dview(&tile_image_views[0])
                .build(),
            descriptor_set_layout.build_descriptor_set()?
                .uniform_buffer(&uniform_buffer)
                .image2dview(&tile_image_views[1])
                .build(),
        ];

        // create pipeline
        let shader = gpu.create_compute_shader(&code)?;
        let pipeline = pipeline_layout.create_compute_pipeline(&shader)?;

        // build command buffers
        let command_buffers = [
            queue.create_command_buffer()?,
            queue.create_command_buffer()?,
        ];
        for i in 0..2 {
            command_buffers[i].begin()?;
            command_buffers[i].bind_compute_pipeline(&pipeline);
            command_buffers[i].bind_compute_descriptor_set(&pipeline_layout,0,&descriptor_sets[i]);
            command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0, });
            command_buffers[i].dispatch(config.tile_size.x as usize,config.tile_size.y as usize,1);
            command_buffers[i].end()?;
        }

        Ok(Tiler {
            _gpu: Arc::clone(&gpu),
            queue,

            config,
            march,
            render,
            uniform_buffer,

            state: State::Idle,

            assemblies,
        
            tile_images,
            _tile_image_views: tile_image_views,
        
            _pipeline_layout: pipeline_layout,
            _descriptor_set_layout: descriptor_set_layout,
            
            _pipeline: pipeline,
            _descriptor_sets: descriptor_sets,
            _shader: shader,
            command_buffers,

            counter: 0,
        })
    }

    pub fn process_command(&mut self,command: Command) -> Result<(),String> {
        match command {
            Command::Execute(config,march,render) => {
                self.config = config;
                self.march = march;
                self.render = render;
                self.state = State::Rendering;
            },
            Command::Exit => {
                self.state = State::Exiting;
            },
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(),String> {
        if let State::Rendering = self.state {

            let stereo = (self.config.flags & TILER_FLAGS_STEREO) != 0;
            self.uniform_buffer.write(&self.queue)?[0] = Uniforms {
                config: self.config,
                march: self.march,
                render: self.render,
            };
            self.queue.submit(&self.command_buffers[0],None,None)?;
            if stereo {
                self.queue.submit(&self.command_buffers[1],None,None)?;
            }
            self.queue.wait()?;

            for i in 0..if stereo { 2 } else { 1 } {
                let tile_data = self.tile_images[i].read::<u32>(&self.queue,0,0)?;
                for y in 0..self.config.tile_size.y {
                    let length = self.config.tile_size.x as usize;
                    let source_start = (y * self.config.tile_size.x) as usize;
                    let source_slice = &tile_data[source_start..source_start + length];
                    let target_start = (((self.config.current_tile.y * self.config.tile_size.y + y) * self.config.tile_count.x + self.config.current_tile.x) * self.config.tile_size.x) as usize;
                    let target_slice = &mut self.assemblies[i][target_start..target_start + length];
                    unsafe { copy_nonoverlapping(source_slice.as_ptr(),target_slice.as_mut_ptr(),length) };
                }
            }
            let ready = (((self.config.current_tile.y * self.config.tile_count.x + self.config.current_tile.x) as f32) / ((self.config.tile_count.x * self.config.tile_count.y) as f32) * 100.0) as usize;
            logd!("rendering... {:3}%",ready);

            // next tile
            if self.config.current_tile.x >= self.config.tile_count.x - 1 {
                if self.config.current_tile.y >= self.config.tile_count.y - 1 {
                    if stereo {
                        let bytes_left = tga::encode(ImageFormat::RGBA8,Vec2 { x: self.config.tile_size.x * self.config.tile_count.x,y: self.config.tile_size.y * self.config.tile_count.y, }.into(),&self.assemblies[0])?;
                        let bytes_right = tga::encode(ImageFormat::RGBA8,Vec2 { x: self.config.tile_size.x * self.config.tile_count.x,y: self.config.tile_size.y * self.config.tile_count.y, }.into(),&self.assemblies[1])?;
                        fs::write(Path::new(&format!("{:05}l.tga",self.counter)),bytes_left).unwrap();
                        fs::write(Path::new(&format!("{:05}r.tga",self.counter)),bytes_right).unwrap();
                    }
                    else {
                        let bytes = tga::encode(ImageFormat::RGBA8,Vec2 { x: self.config.tile_size.x * self.config.tile_count.x,y: self.config.tile_size.y * self.config.tile_count.y, }.into(),&self.assemblies[0])?;
                        fs::write(Path::new(&format!("{:05}.tga",self.counter)),bytes).unwrap();
                    }
                    logd!("saved {:05}.",self.counter);
                    self.counter += 1;
                    self.state = State::Idle;
                }
                else {
                    self.config.current_tile.x = 0;
                    self.config.current_tile.y += 1;
                }
            }
            else {
                self.config.current_tile.x += 1;
            }
        }
        Ok(())
    }
}
