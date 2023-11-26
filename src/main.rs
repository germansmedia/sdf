// SDF - render fractals in VR
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

mod engine;
use engine::*;

const FORWARD_SPEED: f32 = 0.02;
const STRAFE_SPEED: f32 = 0.01;

#[derive(Clone,Copy,Debug)]
#[repr(C)]
struct Uniforms {
    matrix: Mat4x4<f32>,
    fovs: [Fov<f32>; 2],
}

#[derive(Vertex)]
struct FlatVertex {
    _pos: Vec2<f32>,
}

struct Context {
    command_buffer: Arc<CommandBuffer>,
    _framebuffer: Arc<Framebuffer>,
}

app!(sdf2);
fn main() -> Result<(),String> {

    let app = App::new(2)?;
    let gpu = app.gpu();
    let queue = gpu.queue(0)?;

    let main_view = app.main_view();
    let swapchain = main_view.swapchain();
    let images = swapchain.images();
    let layers = swapchain.layers();

    // pose inside fractal space
    let mut pose = Pose { p: Vec3 { x: 1.0,y: -2.0,z: 10.0, },o: Quat::ONE, };  // camera relative to fractal

    // head orientation
    let mut orientation = Quat::<f32>::ONE;

    // ray marching parameters
    let mut march = March {
        pose: pose.into(),  // pose in fractal space
        scale: 1.0,  // rendering/lighting scale
        horizon: 20.0,  // maximum distance
        escape: 20.0,  // escape distance
        de_stop: 0.0001,  // "de_stop"
        de_stop_factor: 10.0,  // "de_stop_factor"
        max_steps: 500,  // maximum number of path trace steps
        max_iterations: 20,  // maximum number of iteractions
        tbd0: 0,
    };

    // rendering/lighting parameters
    let render = Render {
        albedo_color: Vec4 { x: 0.1,y: 0.5,z: 0.7,w: 1.0, },  // albedo
        key_light_pos: Vec4 { x: -10.0,y: 20.0,z: 30.0, w: 1.0, },  // key light position in fractal space
        key_light_color: Vec4 { x: 1.1,y: 1.2,z: 0.9, w: 1.0, },  // key light color
        shadow_power: Vec4 { x: 1.0,y: 1.2,z: 1.5, w: 40.0, },  // shadow RGB power (old film effect)
        sky_light_color: Vec4 { x: 0.16,y: 0.20,z: 0.28,w: 1.0, },  // sky light color
        ambient_light_color: Vec4 { x: 0.40,y: 0.28,z: 0.20,w: 1.0, },  // Walmart global illumination color
        background_color: Vec4 { x: 0.02,y: 0.05,z: 0.1,w: 0.5, },  // background color and fog multiplier
        glow_color: Vec4 { x: 0.1,y: 0.2,z: 0.2,w: 0.4, },  // proximity glow color and intensity
    };

    // the equirectangular image
    let rgba_image = gpu.create_image2d::<u8>(ImageFormat::RGBA8SRGB,Vec2 { x: SIZE * 2,y: SIZE, },2,1,1,ImageUsage::SampledStorage,None)?;

    // prepare thread
    let engine_gpu = Arc::clone(&gpu);
    let depth_occlusion_code = app.load_asset("assets","depth_occlusion_cs.spirv")?;
    let lighting_code = app.load_asset("assets","lighting_cs.spirv")?;
    let engine_rgba_image = Arc::clone(&rgba_image);
    let (tx,rx) = mpsc::channel();
    let engine_thread = spawn(move || {
        let mut engine = Engine::new(
            &engine_gpu,
            &depth_occlusion_code,
            &lighting_code,
            &engine_rgba_image,
            march,
            render,
        )?;
        match {
            loop {
                match engine.state {
                    EngineState::Idle => {
                        if let Ok(command) = rx.recv() {
                            engine.process_command(command)?;
                        }
                        else {
                            loge!("command receive error");
                            break;
                        }
                    },
                    EngineState::Rendering(_,_,_) => {
                        for command in rx.try_iter() {
                            engine.process_command(command)?;
                        }
                    },
                    EngineState::Exiting => {
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

    // create generic quad
    let mut vertices = Vec::<FlatVertex>::new();
    vertices.push(FlatVertex { _pos: Vec2 { x: -1.0,y: -1.0, }, });
    vertices.push(FlatVertex { _pos: Vec2 { x: 1.0,y: -1.0, }, });
    vertices.push(FlatVertex { _pos: Vec2 { x: 1.0,y: 1.0, }, });
    vertices.push(FlatVertex { _pos: Vec2 { x: -1.0,y: 1.0, }, });
    let generic_quad = gpu.create_vertex_buffer(Init::Data(&vertices))?;

    // create render pass to display rgba_image around user
    let render_pass = gpu.create_color_render_pass(ImageFormat::RGBA8SRGB,1)?;
    let descriptor_set_layout = gpu.build_descriptor_set_layout()
        .uniform_buffer()
        .sampled_image2dview()
        .build()?;
    let mut rgba_image_views = Vec::<Arc<Image2DView>>::new();
    for i in 0..layers {
        rgba_image_views.push(rgba_image.create_view(i,0,1)?);
    }
    let uniforms = Uniforms {
        matrix: Mat4x4::ONE,
        fovs: [Fov { l: 0.0,r: 0.0,b: 0.0,t: 0.0, }; 2],
    };
    let uniform_buffer = gpu.create_uniform_buffer(Init::Data(&[uniforms]))?;
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
    let mut descriptor_sets = Vec::<Arc<DescriptorSet>>::new();
    for i in 0..layers {
        descriptor_sets.push(descriptor_set_layout.build_descriptor_set()?
            .uniform_buffer(&uniform_buffer)
            .sampled_image2dview(&rgba_image_views[i],&sampler)
            .build()
        );
    }
    let pipeline_layout = gpu.create_pipeline_layout(&[&descriptor_set_layout],size_of::<Push>())?;
    let code = app.load_asset("assets","draw_equirect_vs.spirv")?;
    let vertex_shader = gpu.create_vertex_shader(&code)?;
    let code = app.load_asset("assets","draw_equirect_fs.spirv")?;
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

    // create command buffers for each swapchain image
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
            command_buffer.bind_graphics_descriptor_set(&pipeline_layout,0,&descriptor_sets[i]);
            command_buffer.bind_vertex_buffer(&generic_quad);
            command_buffer.push_constants(&pipeline_layout,&Push { eye: i as u32,face: 0,anisotropy: EquirectAnisotropy::Square,y_offset: 0, });
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

    // prepare XR actions
    let action_set = app.create_action_set("action_set")?;
    //let action_exit = action_set.create_bool_action("exit","/user/hand/left/input/x/click")?;
    //let action_navigate = action_set.create_vec2_action("navigate","/user/hand/left/input/thumbstick")?;
    let action_exit = action_set.create_bool_action("exit","/user/hand/right/input/a/click")?;
    let action_navigate = action_set.create_vec2_action("navigate","/user/hand/right/input/thumbstick")?;
    app.attach_action_set(&action_set)?;

    // and go...
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
                        orientation = app.local_space().locate_other(&app.head_space(),t)?.o;
                        let matrix = Mat4x4::<f32>::from(orientation).inv().transpose();
                        uniform_buffer.data_mut()?[0] = Uniforms {
                            matrix,
                            fovs: [main_view.fov(0),main_view.fov(1)],
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

                if let AppState::Focused = app.state() {

                    app.sync_actions(&action_set)?;

                    // exit button
                    let exit = action_exit.get_bool()?;
                    if exit {
                        is_running = false;
                    }

                    // calculate new position from thumbstick
                    let nav = action_navigate.get_vec2()?;
                    if (nav.x != 0.0) || (nav.y != 0.0) {
                        let rotation = Mat3x3::<f32>::from(orientation).inv().transpose();
                        let forward = rotation * Vec3::<f32>::UNIT_Z;
                        let right = rotation * Vec3::<f32>::UNIT_X;
                        pose.p += -FORWARD_SPEED * nav.y * forward + STRAFE_SPEED * nav.x * right;
                        march.pose = pose.into();
                        tx.send(EngineCommand::Update(march,render)).unwrap();
                    }
                }
            },
        }
    }

    let _ = tx.send(EngineCommand::Exit);
    let _ = engine_thread.join();

    gpu.wait_idle();

    Ok(())
}