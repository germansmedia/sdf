// NXR - app that renders a fractal to a stereo cubemap around the user
// by Desmond Germans, 2023

use {
    nxr::*,
    macros::*,
    std::{
        result::Result,
        sync::{
            Arc,
            mpsc,
        },
        thread::spawn,
        mem::size_of,
    },
};

const CUBE_SIZE: usize = 2048;

const PROGRESS_FULL16X16: (u32,u32) = (0,0);
const PROGRESS_RIGHT8X16: (u32,u32) = (8,8);
const PROGRESS_BOTTOM8X8: [(u32,u32); 2] = [(8,0),(0,8),];
const PROGRESS_RIGHT4X8: [(u32,u32); 4] = [(4,4),(12,12),(12,4),(4,12),];
const PROGRESS_BOTTOM4X4: [(u32,u32); 8] = [(4,0),(12,8),(12,0),(4,8),(0,4),(8,12),(8,4),(0,12),];
const PROGRESS_RIGHT2X4: [(u32,u32); 16] = [
    (2,2),(10,10),(10,2),(2,10),(6,6),(14,14),(14,6),(6,14),(6,2),(14,10),(14,2),(6,10),(2,6),(10,14),(10,6),(2,14),
];
const PROGRESS_BOTTOM2X2: [(u32,u32); 32] = [
    (2,0),(10,8),(10,0),(2,8), (6,4),(14,12),(14,4),(6,12),(6,0),(14,8), (14,0),(6,8), (2,4),(10,12),(10,4),(2,12),
    (0,2),(8,10),(8,2), (0,10),(4,6),(12,14),(12,6),(4,14),(4,2),(12,10),(12,2),(4,10),(0,6),(8,14), (8,6), (0,14),
];
const PROGRESS_RIGHT1X2: [(u32,u32); 64] = [
    (1,1),(9,9),  (9,1), (1,9), (5,5),(13,13),(13,5),(5,13),(5,1),(13,9), (13,1),(5,9), (1,5),(9,13), (9,5), (1,13),
    (3,3),(11,11),(11,3),(3,11),(7,7),(15,15),(15,7),(7,15),(7,3),(15,11),(15,3),(7,11),(3,7),(11,15),(11,7),(3,15),
    (3,1),(11,9), (11,1),(3,9), (7,5),(15,13),(15,5),(7,13),(7,1),(15,9), (15,1),(7,9), (3,5),(11,13),(11,5),(3,13),
    (1,3),(9,11), (9,3), (1,11),(5,7),(13,15),(13,7),(5,15),(5,3),(13,11),(13,3),(5,11),(1,7),(9,15), (9,7), (1,15),
];
const PROGRESS_BOTTOM1X1: [(u32,u32); 128] = [
    (1,0),(9,8),  (9,0), (1,8), (5,4),(13,12),(13,4),(5,12), (5,0),(13,8), (13,0),(5,8), (1,4),(9,12), (9,4), (1,12),
    (3,2),(11,10),(11,2),(3,10),(7,6),(15,14),(15,6),(7,14), (7,2),(15,10),(15,2),(7,10),(3,6),(11,14),(11,6),(3,14),
    (3,0),(11,8), (11,0),(3,8), (7,4),(15,12),(15,4),(7,12), (7,0),(15,8), (15,0),(7,8), (3,4),(11,12),(11,4),(3,12),
    (1,2),(9,10), (9,2), (1,10),(5,6),(13,14),(13,6),(5,14), (5,2),(13,10),(13,2),(5,10),(1,6),(9,14), (9,6), (1,14),
    (0,1),(8,9),  (8,1), (0,9), (4,5),(12,13),(12,5),(4,13), (4,1),(12,9), (12,1),(4,9), (0,5),(8,13), (8,5), (0,13),
    (2,3),(10,11),(10,3),(2,11),(6,7),(14,15),(14,7),(6,15), (6,3),(14,11),(14,3),(6,11),(2,7),(10,15),(10,7),(2,15),
    (2,1),(10,9), (10,1),(2,9), (6,5),(14,13),(14,5),(6,13), (6,1),(14,9), (14,1),(6,9), (2,5),(10,13),(10,5),(2,13),
    (0,3),(8,11), (8,3),(0,11), (4,7),(12,15),(12,7),(4,15), (4,3),(12,11),(12,3),(4,11),(0,7),(8,15), (8,7), (0,15),
];

#[derive(Clone,Copy)]
#[repr(u32)]
enum VisualizationMode {
    Output,
    Depth,
    Normal,
    Occlusion,
    Debug,
}

#[derive(Clone,Copy,Debug)]
#[repr(u32)]
enum Progress {
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

#[derive(Clone,Copy)]
#[repr(C)]
struct Params {
    matrix: Mat4x4<f32>,
    scale: f32,
    horizon: f32,
    escape: f32,
    de_stop: f32,
    de_stop_factor: f32,
    unused0: f32,
    unused1: f32,
    unused2: f32,
    cube_size: u32,
    mode: VisualizationMode,
    max_steps: u32,
    max_iterations: u32,
    albedo_color: Vec4<f32>,
    key_light_pos: Vec4<f32>,
    key_light_color: Vec4<f32>,
    shadow_power: Vec4<f32>,
    sky_light_color: Vec4<f32>,
    ambient_light_color: Vec4<f32>,
    background_color: Vec4<f32>,
    glow_color: Vec4<f32>,
}

#[derive(Clone,Copy)]
#[repr(C)]
struct EngineUniforms {
    progress: Progress,
    offset_x: u32,
    offset_y: u32,
    unused0: u32,
    params: Params,
}

#[derive(Clone,Copy)]
#[repr(C)]
struct EngineConstants {
    face: u32,
}

#[derive(Clone,Copy)]
#[repr(C)]
struct ProjectUniforms {
    local_from_left: Mat4x4<f32>,
    local_from_right: Mat4x4<f32>,
    fov_left: Fov<f32>,
    fov_right: Fov<f32>,
}

#[derive(Clone,Copy)]
#[repr(C)]
struct ProjectConstants {
    eye: u32,
}

#[derive(Vertex)]
struct FlatVertex {
    _pos: Vec2<f32>,
}

enum ThreadCommand {
    Params(Params),
    Exit,
}

enum ThreadState {
    Idle,
    Rendering(Progress,usize),
    Exiting,
}

struct Context {
    command_buffer: Arc<CommandBuffer>,
    _framebuffer: Arc<Framebuffer>,
}

struct Engine {
    _gpu: Arc<Gpu>,
    queue: Arc<Queue>,
    _descriptor_set_layout: Arc<DescriptorSetLayout>,  // TODO: these objects are saved because they are needed by other objects, investigate if this can be done smaller by keeping extra copies of the Arcs inside the objects that use them
    _pipeline_layout: Arc<PipelineLayout>,
    _compute_shader: ComputeShader,
    _compute_pipeline: ComputePipeline,
    uniforms: EngineUniforms,
    uniform_buffer: Arc<UniformBuffer>,
    _cubemap: Arc<Image2D>,
    _face_views: [Arc<Image2DView>; 6],
    _descriptor_sets: [Arc<DescriptorSet>; 6],
    command_buffers: [Arc<CommandBuffer>; 6],
    state: ThreadState,
}

impl Engine {
    fn new(gpu: &Arc<Gpu>,code: &[u8],cubemap: &Arc<Image2D>,uniforms: EngineUniforms) -> Result<Engine,String> {
        let queue = gpu.queue(1)?;
        queue.transition_image2d_layout(&cubemap,PipelineStage::ColorAttachmentOutput,PipelineStage::ColorAttachmentOutput,ImageLayout::General,0,6)?;
        let compute_shader = gpu.create_compute_shader(&code)?;
        let descriptor_set_layout = gpu.build_descriptor_set_layout()
            .uniform_buffer()
            .image2dview()
            .build()?;
        let pipeline_layout = gpu.create_pipeline_layout(&[&descriptor_set_layout],size_of::<EngineConstants>())?;
        let compute_pipeline = pipeline_layout.create_compute_pipeline(&compute_shader)?;
        let uniform_buffer = gpu.create_uniform_buffer(Init::Data(&[uniforms]))?;
        let face_views = [
            cubemap.create_view(0,0,1)?,
            cubemap.create_view(1,0,1)?,
            cubemap.create_view(2,0,1)?,
            cubemap.create_view(3,0,1)?,
            cubemap.create_view(4,0,1)?,
            cubemap.create_view(5,0,1)?,
        ];
        let descriptor_sets = [
            descriptor_set_layout.create_descriptor_set()?.uniform_buffer(&uniform_buffer).image2dview(&face_views[0]).build(),
            descriptor_set_layout.create_descriptor_set()?.uniform_buffer(&uniform_buffer).image2dview(&face_views[1]).build(),
            descriptor_set_layout.create_descriptor_set()?.uniform_buffer(&uniform_buffer).image2dview(&face_views[2]).build(),
            descriptor_set_layout.create_descriptor_set()?.uniform_buffer(&uniform_buffer).image2dview(&face_views[3]).build(),
            descriptor_set_layout.create_descriptor_set()?.uniform_buffer(&uniform_buffer).image2dview(&face_views[4]).build(),
            descriptor_set_layout.create_descriptor_set()?.uniform_buffer(&uniform_buffer).image2dview(&face_views[5]).build(),
        ];
        let command_buffers = [
            queue.create_command_buffer()?,
            queue.create_command_buffer()?,
            queue.create_command_buffer()?,
            queue.create_command_buffer()?,
            queue.create_command_buffer()?,
            queue.create_command_buffer()?,
        ];
        for i in 0..6 {
            command_buffers[i].begin()?;
            command_buffers[i].bind_compute_pipeline(&compute_pipeline);
            command_buffers[i].bind_compute_descriptor_set(&pipeline_layout,0,&descriptor_sets[i]);
            command_buffers[i].push_constants(&pipeline_layout,&EngineConstants { face: i as u32, });
            command_buffers[i].dispatch(CUBE_SIZE >> 4,CUBE_SIZE >> 4,1);
            command_buffers[i].end()?;
        }
        Ok(Engine {
            _gpu: Arc::clone(&gpu),
            queue,
            _descriptor_set_layout: descriptor_set_layout,
            _pipeline_layout: pipeline_layout,
            _compute_shader: compute_shader,
            _compute_pipeline: compute_pipeline,
            uniforms,
            uniform_buffer,
            _cubemap: Arc::clone(&cubemap),
            _face_views: face_views,
            _descriptor_sets: descriptor_sets,
            command_buffers,
            state: ThreadState::Rendering(Progress::Full16x16,0),
        })
    }

    fn process_command(&mut self,command: ThreadCommand) -> Result<(),String> {
        match command {
            ThreadCommand::Params(new_params) => {
                self.uniforms.params = new_params;
                self.state = ThreadState::Rendering(Progress::Full16x16,0);
            },
            ThreadCommand::Exit => {
                self.state = ThreadState::Exiting;
            },
        }
        Ok(())
    }

    fn render(&mut self) -> Result<(),String> {
        if let ThreadState::Rendering(progress,pass) = self.state {
            let offset = match progress {
                Progress::Full16x16 => PROGRESS_FULL16X16,
                Progress::Right8x16 => PROGRESS_RIGHT8X16,
                Progress::Bottom8x8 => PROGRESS_BOTTOM8X8[pass],
                Progress::Right4x8 => PROGRESS_RIGHT4X8[pass],
                Progress::Bottom4x4 => PROGRESS_BOTTOM4X4[pass],
                Progress::Right2x4 => PROGRESS_RIGHT2X4[pass],
                Progress::Bottom2x2 => PROGRESS_BOTTOM2X2[pass],
                Progress::Right1x2 => PROGRESS_RIGHT1X2[pass],
                Progress::Bottom1x1 => PROGRESS_BOTTOM1X1[pass],
            };
            self.uniforms.progress = progress;
            self.uniforms.offset_x = offset.0;
            self.uniforms.offset_y = offset.1;
            self.uniform_buffer.data_mut()?[0] = self.uniforms;
            for i in 0..6 {
                logd!("{:?}, pass {}, face {}",progress,pass,i);
                self.queue.submit(&self.command_buffers[i],None,None)?;
            }
            self.queue.wait()?;
            self.state = match progress {
                Progress::Full16x16 => ThreadState::Rendering(Progress::Right8x16,0),
                Progress::Right8x16 => ThreadState::Rendering(Progress::Bottom8x8,0),
                Progress::Bottom8x8 => if pass >= 1 { ThreadState::Rendering(Progress::Right4x8,0) } else { ThreadState::Rendering(Progress::Bottom8x8,pass + 1) },
                Progress::Right4x8 => if pass >= 3 { ThreadState::Rendering(Progress::Bottom4x4,0) } else { ThreadState::Rendering(Progress::Right4x8,pass + 1) },
                Progress::Bottom4x4 => if pass >= 7 { ThreadState::Rendering(Progress::Right2x4,0) } else { ThreadState::Rendering(Progress::Bottom4x4,pass + 1) },
                Progress::Right2x4 => if pass >= 15 { ThreadState::Rendering(Progress::Bottom2x2,0) } else { ThreadState::Rendering(Progress::Right2x4,pass + 1) },
                Progress::Bottom2x2 => if pass >= 31 { ThreadState::Rendering(Progress::Right1x2,0) } else { ThreadState::Rendering(Progress::Bottom2x2,pass + 1) },
                Progress::Right1x2 => if pass >= 63 { ThreadState::Rendering(Progress::Bottom1x1,0) } else { ThreadState::Rendering(Progress::Right1x2,pass + 1) },
                Progress::Bottom1x1 => if pass >= 127 { ThreadState::Idle } else { ThreadState::Rendering(Progress::Bottom1x1,pass + 1) },
            };
        }
        Ok(())
    }
}

app!(sdf2);
fn main() -> Result<(),String> {

    let app = App::new(2)?;

    let main_view = app.main_view();
    let swapchain = main_view.swapchain();
    let images = swapchain.images();
    let layers = swapchain.layers();

    let gpu = app.gpu();
    let queue = gpu.queue(0)?;

    let mut pose = Pose { p: Vec3 { x: -6.0,y: -2.0,z: 15.0, },o: Quat::ONE, };  // camera pose

    let cubemap = gpu.create_image2d::<u8>(ImageFormat::RGBA8SRGB,Vec2 { x: CUBE_SIZE,y: CUBE_SIZE, },6,1,1,ImageUsage::SampledStorage,None)?;

    let thread_gpu = Arc::clone(&gpu);
    let thread_code = app.load_asset("nxr-assets","sdf2_engine_cs.spirv")?;
    let thread_cubemap = Arc::clone(&cubemap);
    let engine_uniforms = EngineUniforms {
        progress: Progress::Full16x16,  // rendering progress level (controlled by render thread)
        offset_x: 0,  // rendering progress X-coordinate inside block (controlled by render thread)
        offset_y: 0,  // rendering progress Y-coordinate inside block (controlled by render thread)
        unused0: 0,
        params: Params {
            matrix: pose.into(),  // pose matrix in fractal space
            scale: 1.0,  // rendering/lighting scale
            horizon: 100.0,  // maximum distance
            escape: 20.0,  // escape distance
            de_stop: 0.0001,  // "de_stop"
            de_stop_factor: 10.0,  // "de_stop_factor"
            unused0: 0.0,
            unused1: 0.0,
            unused2: 0.0,
            cube_size: CUBE_SIZE as u32,  // maybe need this...
            mode: VisualizationMode::Output,  // visualization mode
            max_steps: 1000,  // maximum path trace steps
            max_iterations: 20,  // maximum iterations
            albedo_color: Vec4 { x: 0.6,y: 0.6,z: 0.7,w: 1.0, },  // albedo
            key_light_pos: Vec4 { x: -10.0,y: -20.0,z: 40.0, w: 1.0, },  // key light position in fractal space
            key_light_color: Vec4 { x: 1.1,y: 1.2,z: 0.9, w: 1.0, },  // key light color
            shadow_power: Vec4 { x: 1.0,y: 1.2,z: 1.5, w: 40.0, },  // shadow RGB power (old film effect)
            sky_light_color: Vec4 { x: 0.16,y: 0.20,z: 0.28,w: 1.0, },  // sky light color
            ambient_light_color: Vec4 { x: 0.40,y: 0.28,z: 0.20,w: 1.0, },  // Walmart global illumination color
            background_color: Vec4 { x: 0.02,y: 0.05,z: 0.1,w: 1.0, },  // background and fog color
            glow_color: Vec4 { x: 0.2,y: 0.2,z: 0.2,w: 0.4, },  // proximity glow color and intensity
        },
    };
    let thread_uniforms = engine_uniforms;
    let (tx,rx) = mpsc::channel();
    let render_thread = spawn(move || {
        let mut engine = Engine::new(&thread_gpu,&thread_code,&thread_cubemap,thread_uniforms)?;
        match {
            loop {
                match engine.state {
                    ThreadState::Idle => {
                        if let Ok(command) = rx.recv() {
                            engine.process_command(command)?;
                        }
                        else {
                            loge!("command receive error");
                            break;
                        }
                    },
                    ThreadState::Rendering(_,_) => {
                        for command in rx.try_iter() {
                            engine.process_command(command)?;
                        }
                    },
                    ThreadState::Exiting => {
                        break;
                    },
                }
                engine.render()?;
            }
            Result::<(),String>::Ok(())
        } {
            Ok(()) => { },
            Err(error) => { loge!("render thread crashed ({})",error); },
        }
        Result::<(),String>::Ok(())
    });

    let mut vertices = Vec::<FlatVertex>::new();
    vertices.push(FlatVertex { _pos: Vec2 { x: -1.0,y: -1.0, }, });
    vertices.push(FlatVertex { _pos: Vec2 { x: 1.0,y: -1.0, }, });
    vertices.push(FlatVertex { _pos: Vec2 { x: 1.0,y: 1.0, }, });
    vertices.push(FlatVertex { _pos: Vec2 { x: -1.0,y: 1.0, }, });
    let generic_quad = gpu.create_vertex_buffer(Init::Data(&vertices))?;

    let render_pass = gpu.create_color_render_pass(ImageFormat::RGBA8SRGB,1)?;
    let descriptor_set_layout = gpu.build_descriptor_set_layout()
        .uniform_buffer()
        .sampled_imagecubeview()
        .build()?;
    let cube_view = cubemap.create_cube_view(0,0,1)?;
    let uniform_buffer = gpu.create_uniform_buffer(Init::Data(&[ProjectUniforms {
        local_from_left: Mat4x4::ONE,
        local_from_right: Mat4x4::ONE,
        fov_left: Fov { l: 0.0,r: 0.0,b: 0.0,t: 0.0, },
        fov_right: Fov { l: 0.0,r: 0.0,b: 0.0,t: 0.0, },
    }]))?;
    let sampler = gpu.create_sampler(
        SamplerFilter::Linear,
        SamplerFilter::Linear,
        SamplerFilter::Linear,
        AddressMode::ClampToEdge,
        AddressMode::ClampToEdge,
        AddressMode::ClampToEdge,
        0.0,
        Anisotropy::Disabled,
        CompareOp::Always,
        0.0,
        0.0,
        Vec4::<f32> { x: 0.0,y: 0.0,z: 0.0,w: 0.0, },
        false,
    )?;
    let descriptor_set = descriptor_set_layout.build_descriptor_set()?
        .uniform_buffer(&uniform_buffer)
        .sampled_imagecubeview(&cube_view,&sampler)
        .build();
    let pipeline_layout = gpu.create_pipeline_layout(&[&descriptor_set_layout],size_of::<ProjectConstants>())?;
    let code = app.load_asset("nxr-assets","sdf2_project_vs.spirv")?;
    let vertex_shader = gpu.create_vertex_shader(&code)?;
    let code = app.load_asset("nxr-assets","sdf2_project_fs.spirv")?;
    let fragment_shader = gpu.create_fragment_shader(&code)?;
    let graphics_pipeline = pipeline_layout.create_graphics_pipeline::<FlatVertex>(
        &render_pass,
        &vertex_shader,
        &fragment_shader,
        PrimitiveTopology::TriangleFan,
        PrimitiveRestart::Disabled,
        0,
        DepthClamp::Disabled,
        PrimitiveDiscard::Disabled,
        PolygonMode::Fill,
        CullMode::None,
        DepthBias::Disabled,
        1.0,
        1,
        SampleShading::Disabled,
        AlphaToCoverage::Enabled,
        AlphaToOne::Disabled,
        DepthTest::Disabled,
        true,
        StencilTest::Disabled,
        LogicOp::Disabled,
        Blend::Disabled,
        (true,true,true,true),
        Vec4 { x: 0.0,y: 0.0,z: 0.0,w: 1.0, },
    )?;

    let mut contexts = Vec::<Vec<Context>>::new();
    for image in images.iter() {
        let size = image.size();
        let mut layer_contexts = Vec::<Context>::new();
        for i in 0..layers {
            let color_view = image.create_view(i,0,1)?;
            let framebuffer = render_pass.create_color_framebuffer(&color_view,size.into())?;
            let r = Rect::<isize> { o: Vec2::ZERO,s: size.into(), };
            let command_buffer = queue.create_command_buffer()?;
            command_buffer.begin()?;
            command_buffer.bind_graphics_pipeline(&graphics_pipeline);
            command_buffer.set_viewport(r,0.0,1.0);
            command_buffer.set_scissor(r);
            command_buffer.bind_graphics_descriptor_set(&pipeline_layout,0,&descriptor_set);
            command_buffer.bind_vertex_buffer(&generic_quad);
            command_buffer.push_constants(&pipeline_layout,&ProjectConstants { eye: i as u32, });
            command_buffer.begin_render_pass(&framebuffer,r);
            command_buffer.draw(4,1,0,0);
            command_buffer.end_render_pass();
            command_buffer.end()?;
            layer_contexts.push(Context {
                command_buffer,
                _framebuffer: framebuffer,
            });
        }
        contexts.push(layer_contexts);
    }

    let mut is_running = true;
    while is_running {
        app.flush().into_iter().for_each(|(_,event)| {
            logd!("event: {:?}",event);
            match event {
                Event::Quit | Event::Close => { is_running = false; },
                _ => { },
            }
        });
        match app.state() {
            AppState::Suspended | AppState::Running => { },
            AppState::Visible | AppState::Focused => {
                match app.sync()? {
                    RenderHint::Render(t) => {
                        app.begin_frame(t)?;
                        let index = swapchain.acquire()?;
                        let local_from_head = Mat4x4::<f32>::from(app.local_space().locate_other(&app.head_space(),t)?.o).inv().transpose();
                        let fov_left = main_view.fov(0);
                        let fov_right = main_view.fov(1);
                        uniform_buffer.data_mut()?[0] = ProjectUniforms {
                            local_from_left: local_from_head,
                            local_from_right: local_from_head,
                            fov_left,
                            fov_right,
                        };
                        for _ in 0..layers {
                            queue.transition_image2d_layout(&images[index],PipelineStage::ColorAttachmentOutput,PipelineStage::ColorAttachmentOutput,ImageLayout::General,0,1)?;
                        }
                        for i in 0..layers {
                            queue.submit(&contexts[index][i].command_buffer,None,None)?;
                        }
                        for _ in 0..layers {
                            queue.transition_image2d_layout(&images[index],PipelineStage::ColorAttachmentOutput,PipelineStage::ColorAttachmentOutput,ImageLayout::Present,0,1)?;
                        }
                        swapchain.release(index)?;
                        app.end_frame(t,&[&main_view])?;
                    },
                    RenderHint::Drop(t) => {
                        app.begin_frame(t)?;
                        app.end_frame(t,&[&main_view])?;
                    },
                }
            },
        }
    }

    let _ = tx.send(ThreadCommand::Exit);
    let _ = render_thread.join();

    gpu.wait_idle();

    Ok(())
}