// SDF - render fractals in VR
// by Desmond Germans, 2023

use {
    std::{
        result::Result,
        sync::Arc,
        mem::size_of,
    },
    crate::*,
};

pub const SIZE: usize = 2048;

const PHASE_FULL16X16: (u32,u32) = (0,0);
const PHASE_RIGHT8X16: (u32,u32) = (8,8);
const PHASE_BOTTOM8X8: [(u32,u32); 2] = [(8,0),(0,8),];
const PHASE_RIGHT4X8: [(u32,u32); 4] = [(4,4),(12,12),(12,4),(4,12),];
const PHASE_BOTTOM4X4: [(u32,u32); 8] = [(4,0),(12,8),(12,0),(4,8),(0,4),(8,12),(8,4),(0,12),];
const PHASE_RIGHT2X4: [(u32,u32); 16] = [
    (2,2),(10,10),(10,2),(2,10),(6,6),(14,14),(14,6),(6,14),(6,2),(14,10),(14,2),(6,10),(2,6),(10,14),(10,6),(2,14),
];
const PHASE_BOTTOM2X2: [(u32,u32); 32] = [
    (2,0),(10,8),(10,0),(2,8), (6,4),(14,12),(14,4),(6,12),(6,0),(14,8), (14,0),(6,8), (2,4),(10,12),(10,4),(2,12),
    (0,2),(8,10),(8,2), (0,10),(4,6),(12,14),(12,6),(4,14),(4,2),(12,10),(12,2),(4,10),(0,6),(8,14), (8,6), (0,14),
];
const PHASE_RIGHT1X2: [(u32,u32); 64] = [
    (1,1),(9,9),  (9,1), (1,9), (5,5),(13,13),(13,5),(5,13),(5,1),(13,9), (13,1),(5,9), (1,5),(9,13), (9,5), (1,13),
    (3,3),(11,11),(11,3),(3,11),(7,7),(15,15),(15,7),(7,15),(7,3),(15,11),(15,3),(7,11),(3,7),(11,15),(11,7),(3,15),
    (3,1),(11,9), (11,1),(3,9), (7,5),(15,13),(15,5),(7,13),(7,1),(15,9), (15,1),(7,9), (3,5),(11,13),(11,5),(3,13),
    (1,3),(9,11), (9,3), (1,11),(5,7),(13,15),(13,7),(5,15),(5,3),(13,11),(13,3),(5,11),(1,7),(9,15), (9,7), (1,15),
];
const PHASE_BOTTOM1X1: [(u32,u32); 128] = [
    (1,0),(9,8),  (9,0), (1,8), (5,4),(13,12),(13,4),(5,12), (5,0),(13,8), (13,0),(5,8), (1,4),(9,12), (9,4), (1,12),
    (3,2),(11,10),(11,2),(3,10),(7,6),(15,14),(15,6),(7,14), (7,2),(15,10),(15,2),(7,10),(3,6),(11,14),(11,6),(3,14),
    (3,0),(11,8), (11,0),(3,8), (7,4),(15,12),(15,4),(7,12), (7,0),(15,8), (15,0),(7,8), (3,4),(11,12),(11,4),(3,12),
    (1,2),(9,10), (9,2), (1,10),(5,6),(13,14),(13,6),(5,14), (5,2),(13,10),(13,2),(5,10),(1,6),(9,14), (9,6), (1,14),
    (0,1),(8,9),  (8,1), (0,9), (4,5),(12,13),(12,5),(4,13), (4,1),(12,9), (12,1),(4,9), (0,5),(8,13), (8,5), (0,13),
    (2,3),(10,11),(10,3),(2,11),(6,7),(14,15),(14,7),(6,15), (6,3),(14,11),(14,3),(6,11),(2,7),(10,15),(10,7),(2,15),
    (2,1),(10,9), (10,1),(2,9), (6,5),(14,13),(14,5),(6,13), (6,1),(14,9), (14,1),(6,9), (2,5),(10,13),(10,5),(2,13),
    (0,3),(8,11), (8,3),(0,11), (4,7),(12,15),(12,7),(4,15), (4,3),(12,11),(12,3),(4,11),(0,7),(8,15), (8,7), (0,15),
];

#[allow(dead_code)]
#[derive(Clone,Copy,Debug)]
#[repr(u32)]
pub enum ViewType {
    Quad,
    StereoQuad,
    Cube,
    StereoCube,
    Cylinder,
    StereoCylinder,
    Equirect,
    StereoEquirect,
    Fisheye,
    StereoFisheye,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct ViewConfig {
    pub width: u32,
    pub height: u32,
    pub type_: ViewType,
    pub tbd0: u32,
    pub fov: Fov<f32>,
}

#[derive(Clone,Copy,Debug)]
#[repr(u32)]
pub enum Phase {
    Full16x16,
    Right8x16,
    Bottom8x8,
    Right4x8,
    Bottom4x4,
    Right2x4,
    Bottom2x2,
    Right1x2,
    Bottom1x1,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct Progress {
    pub phase: Phase,
    pub offset: Vec2<u32>,
    pub tbd0: u32,
}

#[derive(Clone,Copy,Debug,PartialEq)]
#[repr(C)]
pub struct March {
    pub pose: Mat4x4<f32>,
    pub scale: f32,
    pub horizon: f32,
    pub escape: f32,
    pub de_stop: f32,
    pub de_stop_factor: f32,
    pub max_steps: u32,
    pub max_iterations: u32,
    pub tbd0: u32,
    pub forward_dir: Vec4<f32>,
}

#[derive(Clone,Copy,Debug,PartialEq)]
#[repr(C)]
pub struct Render {
    pub key_light_pos: Vec4<f32>,
    pub key_light_color: Vec4<f32>,
    pub shadow_power: Vec4<f32>,
    pub sky_light_color: Vec4<f32>,

    pub ambient_light_color: Vec4<f32>,
    pub background_color: Vec4<f32>,
    pub glow_color: Vec4<f32>,
    pub tbd0: Vec4<f32>,

    pub palette: [Vec4<f32>; 4],
}

#[derive(Clone,Copy,Debug)]
#[repr(u32)]
pub enum EquirectAnisotropy {
    Square,
    Rect2,
    Rect4,
    Rect8,
    Rect16,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct Push {
    pub eye: u32,
    pub face: u32,
    pub anisotropy: EquirectAnisotropy,
    pub y_offset: u32,
}

pub enum EngineCommand {
    Update(March,Render),
    Exit,
}

#[derive(Clone,Copy,Debug)]
pub enum Stage {
    DepthOcclusion,
    Lighting,
}

pub enum EngineState {
    Idle,
    Rendering(Stage,Phase,usize),
    Exiting,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct EngineUniforms {
    pub view: ViewConfig,
    pub progress: Progress,
    pub march: March,
    pub render: Render,
}

pub struct Engine {
    _gpu: Arc<Gpu>,
    queue: Arc<Queue>,

    march: March,
    render: Render,
    uniform_buffer: Arc<UniformBuffer<EngineUniforms>>,

    pub state: EngineState,

    _dosi_image: Arc<Image2D>,
    _dosi_image_views: [Arc<Image2DView>; 2],
    rgba_image: Arc<Image2D>,
    _rgba_image_views: [Arc<Image2DView>; 2],

    _pipeline_layout: Arc<PipelineLayout>,
    _descriptor_set_layout: Arc<DescriptorSetLayout>,

    _depth_occlusion_pipeline: Arc<ComputePipeline>,
    _depth_occlusion_descriptor_sets: [Arc<DescriptorSet>; 2],
    _depth_occlusion_shader: Arc<ComputeShader>,
    depth_occlusion_command_buffers: [Arc<CommandBuffer>; 2],

    _lighting_pipeline: Arc<ComputePipeline>,
    _lighting_descriptor_sets: [Arc<DescriptorSet>; 2],
    _lighting_shader: Arc<ComputeShader>,
    lighting_command_buffers: [Arc<CommandBuffer>; 2],
}

impl Engine {

    pub fn new(gpu: &Arc<Gpu>,depth_occlusion_code: &[u8],lighting_code: &[u8],rgba_image: &Arc<Image2D>,march: March,render: Render) -> Result<Engine,String> {

        // get engine thread queue
        let queue = gpu.queue(1)?;

        // transition image layout
        queue.transition_image2d_layout(&rgba_image,PipelineStage::ColorAttachmentOutput,PipelineStage::ColorAttachmentOutput,ImageLayout::General,0,2)?;

        // create depth-occlusion image
        let size = rgba_image.size();
        let dosi_image = gpu.create_empty_image2d(ImageFormat::RGBA32F,size,rgba_image.layers(),1,1,ImageUsage::Storage,AccessStyle::Gpu)?;

        // create image views
        let dosi_image_views = [
            dosi_image.create_view(0,0,1)?,
            dosi_image.create_view(1,0,1)?,
        ];
        let rgba_image_views = [
            rgba_image.create_view(0,0,1)?,
            rgba_image.create_view(1,0,1)?,
        ];

        // create pipeline layout
        let descriptor_set_layout = gpu.build_descriptor_set_layout()
            .uniform_buffer()
            .image2dview()
            .image2dview()
            .build()?;
        let pipeline_layout = gpu.create_pipeline_layout(&[&descriptor_set_layout],size_of::<Push>())?;

        // create uniform buffers
        let uniforms = EngineUniforms {
            view: ViewConfig {
                width: size.x as u32,
                height: size.y as u32,
                type_: ViewType::StereoEquirect,
                tbd0: 0,
                fov: Fov { l: 0.0,r: 0.0,b: 0.0,t: 0.0, },
            },
            progress: Progress {
                phase: Phase::Full16x16,
                offset: Vec2::ZERO,
                tbd0: 0,
            },
            march,
            render,
        };
        let uniform_buffer = gpu.create_uniform_buffer(&queue,AccessStyle::Shared,&[uniforms])?;

        // create descriptor sets
        let depth_occlusion_descriptor_sets = [
            descriptor_set_layout.build_descriptor_set()?
                .uniform_buffer(&uniform_buffer)
                .image2dview(&dosi_image_views[0])
                .image2dview(&rgba_image_views[0])
                .build(),
            descriptor_set_layout.build_descriptor_set()?
                .uniform_buffer(&uniform_buffer)
                .image2dview(&dosi_image_views[1])
                .image2dview(&rgba_image_views[1])
                .build(),
        ];
        let lighting_descriptor_sets = [
            descriptor_set_layout.build_descriptor_set()?
                .uniform_buffer(&uniform_buffer)
                .image2dview(&dosi_image_views[0])
                .image2dview(&rgba_image_views[0])
                .build(),
            descriptor_set_layout.build_descriptor_set()?
                .uniform_buffer(&uniform_buffer)
                .image2dview(&dosi_image_views[1])
                .image2dview(&rgba_image_views[1])
                .build(),
        ];

        // create pipelines
        let depth_occlusion_shader = gpu.create_compute_shader(&depth_occlusion_code)?;
        let depth_occlusion_pipeline = pipeline_layout.create_compute_pipeline(&depth_occlusion_shader)?;
        let lighting_shader = gpu.create_compute_shader(&lighting_code)?;
        let lighting_pipeline = pipeline_layout.create_compute_pipeline(&lighting_shader)?;

        // build command buffers
        let depth_occlusion_command_buffers = [
            queue.create_command_buffer()?,
            queue.create_command_buffer()?,
        ];
        let lighting_command_buffers = [
            queue.create_command_buffer()?,
            queue.create_command_buffer()?,
        ];

        // create rendering strips of varying anisotropy
        let width_in_blocks = SIZE / 8;
        let strip_height_in_pixels = SIZE / 32;
        let strip_height_in_blocks = strip_height_in_pixels / 16;
        for i in 0..2 {
            depth_occlusion_command_buffers[i].begin()?;
            depth_occlusion_command_buffers[i].bind_compute_pipeline(&depth_occlusion_pipeline);
            depth_occlusion_command_buffers[i].bind_compute_descriptor_set(&pipeline_layout,0,&depth_occlusion_descriptor_sets[i]);
            depth_occlusion_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect16,y_offset: 0, });
            depth_occlusion_command_buffers[i].dispatch(width_in_blocks / 16,strip_height_in_blocks,1);
            depth_occlusion_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect8,y_offset: strip_height_in_pixels as u32, });
            depth_occlusion_command_buffers[i].dispatch(width_in_blocks / 8,strip_height_in_blocks,1);
            depth_occlusion_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect4,y_offset: 2 * strip_height_in_pixels as u32, });
            depth_occlusion_command_buffers[i].dispatch(width_in_blocks / 4,2 * strip_height_in_blocks,1);
            depth_occlusion_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect2,y_offset: 4 * strip_height_in_pixels as u32, });
            depth_occlusion_command_buffers[i].dispatch(width_in_blocks / 2,4 * strip_height_in_blocks,1);
            depth_occlusion_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Square,y_offset: 8 * strip_height_in_pixels as u32, });
            depth_occlusion_command_buffers[i].dispatch(width_in_blocks,16 * strip_height_in_blocks,1);
            depth_occlusion_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect2,y_offset: 24 * strip_height_in_pixels as u32, });
            depth_occlusion_command_buffers[i].dispatch(width_in_blocks / 2,4 * strip_height_in_blocks,1);
            depth_occlusion_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect4,y_offset: 28 * strip_height_in_pixels as u32, });
            depth_occlusion_command_buffers[i].dispatch(width_in_blocks / 4,2 * strip_height_in_blocks,1);
            depth_occlusion_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect8,y_offset: 30 * strip_height_in_pixels as u32, });
            depth_occlusion_command_buffers[i].dispatch(width_in_blocks / 8,strip_height_in_blocks,1);
            depth_occlusion_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect16,y_offset: 31 * strip_height_in_pixels as u32, });
            depth_occlusion_command_buffers[i].dispatch(width_in_blocks / 16,strip_height_in_blocks,1);
            depth_occlusion_command_buffers[i].end()?;
        }
        for i in 0..2 {
            lighting_command_buffers[i].begin()?;
            lighting_command_buffers[i].bind_compute_pipeline(&lighting_pipeline);
            lighting_command_buffers[i].bind_compute_descriptor_set(&pipeline_layout,0,&lighting_descriptor_sets[i]);
            lighting_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect16,y_offset: 0, });
            lighting_command_buffers[i].dispatch(width_in_blocks / 16,strip_height_in_blocks,1);
            lighting_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect8,y_offset: strip_height_in_pixels as u32, });
            lighting_command_buffers[i].dispatch(width_in_blocks / 8,strip_height_in_blocks,1);
            lighting_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect4,y_offset: 2 * strip_height_in_pixels as u32, });
            lighting_command_buffers[i].dispatch(width_in_blocks / 4,2 * strip_height_in_blocks,1);
            lighting_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect2,y_offset: 4 * strip_height_in_pixels as u32, });
            lighting_command_buffers[i].dispatch(width_in_blocks / 2,4 * strip_height_in_blocks,1);
            lighting_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Square,y_offset: 8 * strip_height_in_pixels as u32, });
            lighting_command_buffers[i].dispatch(width_in_blocks,16 * strip_height_in_blocks,1);
            lighting_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect2,y_offset: 24 * strip_height_in_pixels as u32, });
            lighting_command_buffers[i].dispatch(width_in_blocks / 2,4 * strip_height_in_blocks,1);
            lighting_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect4,y_offset: 28 * strip_height_in_pixels as u32, });
            lighting_command_buffers[i].dispatch(width_in_blocks / 4,2 * strip_height_in_blocks,1);
            lighting_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect8,y_offset: 30 * strip_height_in_pixels as u32, });
            lighting_command_buffers[i].dispatch(width_in_blocks / 8,strip_height_in_blocks,1);
            lighting_command_buffers[i].push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Rect16,y_offset: 31 * strip_height_in_pixels as u32, });
            lighting_command_buffers[i].dispatch(width_in_blocks / 16,strip_height_in_blocks,1);
            lighting_command_buffers[i].end()?;
        }

        Ok(Engine {
            _gpu: Arc::clone(&gpu),
            queue,
        
            march,
            render,
            uniform_buffer,

            state: EngineState::Rendering(Stage::DepthOcclusion,Phase::Full16x16,0),
        
            _dosi_image: dosi_image,
            _dosi_image_views: dosi_image_views,
            rgba_image: Arc::clone(&rgba_image),
            _rgba_image_views: rgba_image_views,
        
            _pipeline_layout: pipeline_layout,
            _descriptor_set_layout: descriptor_set_layout,
            
            _depth_occlusion_pipeline: depth_occlusion_pipeline,
            _depth_occlusion_descriptor_sets: depth_occlusion_descriptor_sets,
            _depth_occlusion_shader: depth_occlusion_shader,
            depth_occlusion_command_buffers,
        
            _lighting_pipeline: lighting_pipeline,
            _lighting_descriptor_sets: lighting_descriptor_sets,
            _lighting_shader: lighting_shader,
            lighting_command_buffers,
        })
    }

    pub fn process_command(&mut self,command: EngineCommand) -> Result<(),String> {
        match command {
            EngineCommand::Update(march,render) => {
                if march != self.march {
                    self.march = march;
                    self.render = render;
                    self.state = EngineState::Rendering(Stage::DepthOcclusion,Phase::Full16x16,0);
                }
                if render != self.render {
                    self.render = render;
                    self.state = EngineState::Rendering(Stage::Lighting,Phase::Full16x16,0);
                }
            },
            EngineCommand::Exit => {
                self.state = EngineState::Exiting;
            },
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(),String> {
        if let EngineState::Rendering(stage,phase,pass) = self.state {
            let offset = match phase {
                Phase::Full16x16 => PHASE_FULL16X16,
                Phase::Right8x16 => PHASE_RIGHT8X16,
                Phase::Bottom8x8 => PHASE_BOTTOM8X8[pass],
                Phase::Right4x8 => PHASE_RIGHT4X8[pass],
                Phase::Bottom4x4 => PHASE_BOTTOM4X4[pass],
                Phase::Right2x4 => PHASE_RIGHT2X4[pass],
                Phase::Bottom2x2 => PHASE_BOTTOM2X2[pass],
                Phase::Right1x2 => PHASE_RIGHT1X2[pass],
                Phase::Bottom1x1 => PHASE_BOTTOM1X1[pass],
            };
            match stage {
                Stage::DepthOcclusion => {
                    self.uniform_buffer.write(&self.queue)?[0] = EngineUniforms {
                        view: ViewConfig {
                            width: self.rgba_image.size().x as u32,
                            height: self.rgba_image.size().y as u32,
                            type_: ViewType::StereoEquirect,
                            tbd0: 0,
                            fov: Fov { l: 0.0,r: 0.0,b: 0.0,t: 0.0, },
                        },
                        progress: Progress {
                            phase,
                            offset: Vec2 { x: offset.0,y: offset.1, },
                            tbd0: 0,
                        },
                        march: self.march,
                        render: self.render,
                    };
                    for i in 0..2 {
                        self.queue.submit(&self.depth_occlusion_command_buffers[i],None,None)?;
                    }
                    self.queue.wait()?;
                    self.state = match phase {
                        Phase::Full16x16 => EngineState::Rendering(Stage::DepthOcclusion,Phase::Right8x16,0),
                        Phase::Right8x16 => EngineState::Rendering(Stage::DepthOcclusion,Phase::Bottom8x8,0),
                        Phase::Bottom8x8 => if pass >= 1 { EngineState::Rendering(Stage::DepthOcclusion,Phase::Right4x8,0) } else { EngineState::Rendering(Stage::DepthOcclusion,Phase::Bottom8x8,pass + 1) },
                        Phase::Right4x8 => if pass >= 3 { EngineState::Rendering(Stage::DepthOcclusion,Phase::Bottom4x4,0) } else { EngineState::Rendering(Stage::DepthOcclusion,Phase::Right4x8,pass + 1) },
                        Phase::Bottom4x4 => if pass >= 7 { EngineState::Rendering(Stage::DepthOcclusion,Phase::Right2x4,0) } else { EngineState::Rendering(Stage::DepthOcclusion,Phase::Bottom4x4,pass + 1) },
                        Phase::Right2x4 => if pass >= 15 { EngineState::Rendering(Stage::DepthOcclusion,Phase::Bottom2x2,0) } else { EngineState::Rendering(Stage::DepthOcclusion,Phase::Right2x4,pass + 1) },
                        Phase::Bottom2x2 => if pass >= 31 { EngineState::Rendering(Stage::DepthOcclusion,Phase::Right1x2,0) } else { EngineState::Rendering(Stage::DepthOcclusion,Phase::Bottom2x2,pass + 1) },
                        Phase::Right1x2 => if pass >= 63 { EngineState::Rendering(Stage::DepthOcclusion,Phase::Bottom1x1,0) } else { EngineState::Rendering(Stage::DepthOcclusion,Phase::Right1x2,pass + 1) },
                        Phase::Bottom1x1 => if pass >= 127 { EngineState::Rendering(Stage::Lighting,Phase::Full16x16,0) } else { EngineState::Rendering(Stage::DepthOcclusion,Phase::Bottom1x1,pass + 1) },
                    };        
                },
                Stage::Lighting => {
                    self.uniform_buffer.write(&self.queue)?[0] = EngineUniforms {
                        view: ViewConfig {
                            width: self.rgba_image.size().x as u32,
                            height: self.rgba_image.size().y as u32,
                            type_: ViewType::StereoEquirect,
                            tbd0: 0,
                            fov: Fov { l: 0.0,r: 0.0,b: 0.0,t: 0.0, },
                        },
                        progress: Progress {
                            phase,
                            offset: Vec2 { x: offset.0,y: offset.1, },
                            tbd0: 0,
                        },
                        march: self.march,
                        render: self.render,
                    };
                    for i in 0..2 {
                        self.queue.submit(&self.lighting_command_buffers[i],None,None)?;
                    }
                    self.queue.wait()?;
                    self.state = match phase {
                        Phase::Full16x16 => EngineState::Rendering(Stage::Lighting,Phase::Right8x16,0),
                        Phase::Right8x16 => EngineState::Rendering(Stage::Lighting,Phase::Bottom8x8,0),
                        Phase::Bottom8x8 => if pass >= 1 { EngineState::Rendering(Stage::Lighting,Phase::Right4x8,0) } else { EngineState::Rendering(Stage::Lighting,Phase::Bottom8x8,pass + 1) },
                        Phase::Right4x8 => if pass >= 3 { EngineState::Rendering(Stage::Lighting,Phase::Bottom4x4,0) } else { EngineState::Rendering(Stage::Lighting,Phase::Right4x8,pass + 1) },
                        Phase::Bottom4x4 => if pass >= 7 { EngineState::Rendering(Stage::Lighting,Phase::Right2x4,0) } else { EngineState::Rendering(Stage::Lighting,Phase::Bottom4x4,pass + 1) },
                        Phase::Right2x4 => if pass >= 15 { EngineState::Rendering(Stage::Lighting,Phase::Bottom2x2,0) } else { EngineState::Rendering(Stage::Lighting,Phase::Right2x4,pass + 1) },
                        Phase::Bottom2x2 => if pass >= 31 { EngineState::Rendering(Stage::Lighting,Phase::Right1x2,0) } else { EngineState::Rendering(Stage::Lighting,Phase::Bottom2x2,pass + 1) },
                        Phase::Right1x2 => if pass >= 63 { EngineState::Rendering(Stage::Lighting,Phase::Bottom1x1,0) } else { EngineState::Rendering(Stage::Lighting,Phase::Right1x2,pass + 1) },
                        Phase::Bottom1x1 => if pass >= 127 { EngineState::Idle } else { EngineState::Rendering(Stage::Lighting,Phase::Bottom1x1,pass + 1) },
                    };        
                },
            }
        }
        Ok(())
    }
}
