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
    
    pub fov: Fov<f32>,
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

    assembly: Vec<u32>,

    tile_image: Arc<Image2D>,
    _tile_image_view: Arc<Image2DView>,

    _pipeline_layout: Arc<PipelineLayout>,
    _descriptor_set_layout: Arc<DescriptorSetLayout>,

    _pipeline: Arc<ComputePipeline>,
    _descriptor_set: Arc<DescriptorSet>,
    _shader: Arc<ComputeShader>,
    command_buffer: Arc<CommandBuffer>,
}

impl Tiler {

    pub fn new(gpu: &Arc<Gpu>,code: &[u8],config: Config,march: March,render: Render) -> Result<Tiler,String> {

        // get engine thread queue
        let queue = gpu.queue(2)?;

        // create tile image
        let tile_image = gpu.create_empty_image2d(ImageFormat::RGBA8SRGB,config.tile_size.into(),1,1,1,ImageUsage::Storage,AccessStyle::Shared)?;

        // create assembly stage
        let assembly = vec![0u32; (config.tile_size.x * config.tile_size.y * config.tile_count.x * config.tile_count.y) as usize];

        // transition image layout
        queue.transition_image2d_layout(&tile_image,PipelineStage::ColorAttachmentOutput,PipelineStage::ColorAttachmentOutput,ImageLayout::General,0,2)?;

        // create image view
        let tile_image_view = tile_image.create_view(0,0,1)?;

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

        // create descriptor set
        let descriptor_set = descriptor_set_layout.build_descriptor_set()?
            .uniform_buffer(&uniform_buffer)
            .image2dview(&tile_image_view)
            .build();

        // create pipelines
        let shader = gpu.create_compute_shader(&code)?;
        let pipeline = pipeline_layout.create_compute_pipeline(&shader)?;

        // build command buffer
        let command_buffer = queue.create_command_buffer()?;
        command_buffer.begin()?;
        command_buffer.bind_compute_pipeline(&pipeline);
        command_buffer.bind_compute_descriptor_set(&pipeline_layout,0,&descriptor_set);
        command_buffer.dispatch(config.tile_size.x as usize,config.tile_size.y as usize,1);
        command_buffer.end()?;

        Ok(Tiler {
            _gpu: Arc::clone(&gpu),
            queue,

            config,
            march,
            render,
            uniform_buffer,

            state: State::Idle,

            assembly,
        
            tile_image: Arc::clone(&tile_image),
            _tile_image_view: tile_image_view,
        
            _pipeline_layout: pipeline_layout,
            _descriptor_set_layout: descriptor_set_layout,
            
            _pipeline: pipeline,
            _descriptor_set: descriptor_set,
            _shader: shader,
            command_buffer,
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
            logd!("uniform config fov = {}",self.config.fov);
            self.uniform_buffer.write(&self.queue)?[0] = Uniforms {
                config: self.config,
                march: self.march,
                render: self.render,
            };
            self.queue.submit(&self.command_buffer,None,None)?;
            self.queue.wait()?;

            // copy tile into assembly
            {
                let tile_data = self.tile_image.read::<u32>(&self.queue,0,0)?;
                for y in 0..self.config.tile_size.y {
                    let length = self.config.tile_size.x as usize;
                    let source_start = (y * self.config.tile_size.x) as usize;
                    let source_slice = &tile_data[source_start..source_start + length];
                    let target_start = (((self.config.current_tile.y * self.config.tile_size.y + y) * self.config.tile_count.x + self.config.current_tile.x) * self.config.tile_size.x) as usize;
                    let target_slice = &mut self.assembly[target_start..target_start + length];
                    unsafe { copy_nonoverlapping(source_slice.as_ptr(),target_slice.as_mut_ptr(),length) };
                }
            }
            logd!("tile {},{} ready.",self.config.current_tile.x,self.config.current_tile.y);

            // next tile
            if self.config.current_tile.x >= self.config.tile_count.x - 1 {
                if self.config.current_tile.y >= self.config.tile_count.y - 1 {
                    let bytes = tga::encode(ImageFormat::RGBA8,Vec2 { x: self.config.tile_size.x * self.config.tile_count.x,y: self.config.tile_size.y * self.config.tile_count.y, }.into(),&self.assembly)?;
                    fs::write(Path::new("test.tga"),bytes).unwrap();
                    logd!("saved to test.tga.");
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
