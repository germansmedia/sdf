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

const FRAMES: usize = 300;

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
    pub params: Params,
}

pub enum Command {
    Photo(Config,Params),
    Video(Config,Params,Params),
    Exit,
}

pub enum State {
    Idle,
    RenderingPhoto,
    RenderingVideo,
    Exiting,
}

pub struct Tiler {
    _gpu: Arc<Gpu>,
    queue: Arc<Queue>,

    config: Config,
    params: Params,
    keyframes: [Params; 2],
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

    frame: usize,
}

fn interpolate_params(start: Params,end: Params,f: f32) -> Params {
    let oos = (1.0 - f) / start.scale + f / end.scale;
    Params {
        pose: ((1.0 - f) * start.pose / start.scale + f * end.pose / end.scale) / oos,
        forward_dir: start.forward_dir,
        key_light_pos: (1.0 - f) * start.key_light_pos + f * end.key_light_pos,
        key_light_color: (1.0 - f) * start.key_light_color + f * end.key_light_color,
        shadow_power: (1.0 - f) * start.shadow_power + f * end.shadow_power,
        sky_light_color: (1.0 - f) * start.sky_light_color + f * end.sky_light_color,
        ambient_light_color: (1.0 - f) * start.ambient_light_color + f * end.ambient_light_color,
        background_color: start.background_color,
        glow_color: (1.0 - f) * start.glow_color + f * end.glow_color,
        palette: start.palette,
        scale: (1.0 - f) * start.scale + f * end.scale,
        horizon: start.horizon,
        escape: start.escape,
        dtf_const: start.dtf_const,
        dtf_linear: start.dtf_linear,
        max_steps: start.max_steps,
        max_iterations: start.max_iterations,
        step_size: start.step_size,
        iod: start.iod,
        tbd0: 0,
        tbd1: 0,
        tbd2: 0,
    }
}

impl Tiler {

    pub fn new(gpu: &Arc<Gpu>,code: &[u8],config: Config,params: Params) -> Result<Tiler,String> {

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
            params,
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
            params,
            keyframes: [params,params],
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

            frame: 0,
        })
    }

    pub fn process_command(&mut self,command: Command) -> Result<(),String> {
        match command {
            Command::Photo(config,params) => {
                self.config = config;
                self.params = params;
                self.state = State::RenderingPhoto;
            },
            Command::Video(config,start,end) => {
                self.config = config;
                self.params = start;
                self.keyframes[0] = start;
                self.keyframes[1] = end;
                self.frame = 0;
                self.state = State::RenderingVideo;
            },
            Command::Exit => {
                self.state = State::Exiting;
            },
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(),String> {
        match self.state {

            State::RenderingPhoto | State::RenderingVideo => {

                // render current tile
                let stereo = (self.config.flags & TILER_FLAGS_STEREO) != 0;
                self.uniform_buffer.write(&self.queue)?[0] = Uniforms {
                    config: self.config,
                    params: self.params,
                };
                self.queue.submit(&self.command_buffers[0],None,None)?;
                if stereo {
                    self.queue.submit(&self.command_buffers[1],None,None)?;
                }
                self.queue.wait()?;

                // copy tile into assembly
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
                if let State::RenderingPhoto = self.state {
                    let ready = (((self.config.current_tile.y * self.config.tile_count.x + self.config.current_tile.x) as f32) / ((self.config.tile_count.x * self.config.tile_count.y) as f32) * 100.0) as usize;
                    logd!("rendering photo... {:3}%",ready);
                }

                // next tile
                if self.config.current_tile.x >= self.config.tile_count.x - 1 {
                    if self.config.current_tile.y >= self.config.tile_count.y - 1 {

                        // image is ready, save it
                        if stereo {
                            let bytes_left = tga::encode(ImageFormat::RGBA8,Vec2 { x: self.config.tile_size.x * self.config.tile_count.x,y: self.config.tile_size.y * self.config.tile_count.y, }.into(),&self.assemblies[0])?;
                            let bytes_right = tga::encode(ImageFormat::RGBA8,Vec2 { x: self.config.tile_size.x * self.config.tile_count.x,y: self.config.tile_size.y * self.config.tile_count.y, }.into(),&self.assemblies[1])?;
                            match self.state {
                                State::RenderingPhoto => {
                                    fs::write(Path::new(&format!("{:03}l.tga",self.counter)),bytes_left).unwrap();
                                    fs::write(Path::new(&format!("{:03}r.tga",self.counter)),bytes_right).unwrap();        
                                },
                                State::RenderingVideo => {
                                    fs::write(Path::new(&format!("{:03}-{:05}l.tga",self.counter,self.frame)),bytes_left).unwrap();
                                    fs::write(Path::new(&format!("{:03}-{:05}r.tga",self.counter,self.frame)),bytes_right).unwrap();        
                                },
                                State::Idle | State::Exiting => { },
                            }
                        }
                        else {
                            let bytes = tga::encode(ImageFormat::RGBA8,Vec2 { x: self.config.tile_size.x * self.config.tile_count.x,y: self.config.tile_size.y * self.config.tile_count.y, }.into(),&self.assemblies[0])?;
                            match self.state {
                                State::RenderingPhoto => {
                                    fs::write(Path::new(&format!("{:05}.tga",self.counter)),bytes).unwrap();
                                },
                                State::RenderingVideo => {
                                    fs::write(Path::new(&format!("{:03}-{:05}.tga",self.counter,self.frame)),bytes).unwrap();
                                },
                                State::Idle | State::Exiting => { },
                            }
                        }
                        
                        match self.state {
                            State::RenderingPhoto => {
                                logd!("saved photo {:05}.",self.counter);
                                self.counter += 1;
                                self.state = State::Idle;        
                            },
                            State::RenderingVideo => {
                                let ready = ((self.frame as f32) / (FRAMES as f32) * 100.0) as usize;
                                logd!("rendering video... {:3}%",ready);
                                if self.frame >= FRAMES - 1 {
                                    logd!("rendered video.");
                                    self.counter += 1;
                                    self.state = State::Idle;
                                }
                                else {
                                    self.frame += 1;
                                    self.params = interpolate_params(self.keyframes[0],self.keyframes[1],(self.frame as f32) / (FRAMES as f32));
                                    self.config.current_tile.x = 0;
                                    self.config.current_tile.y = 0;
                                }
                            },
                            State::Idle | State::Exiting => { },
                        }
                    }
                    else {
                        self.config.current_tile.x = 0;
                        self.config.current_tile.y += 1;
                    }
                }
                else {
                    self.config.current_tile.x += 1;
                }
            },

            State::Idle | State::Exiting => { },
        }

        Ok(())
    }
}
