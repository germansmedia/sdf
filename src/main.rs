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
    },
};

mod engine;
use engine::*;

mod projector;
use projector::*;

mod yardstick;
use yardstick::*;

const FORWARD_SPEED: f32 = 0.02;
const STRAFE_SPEED: f32 = 0.01;

app!(sdf2);
fn main() -> Result<(),String> {

    let app = App::new(2)?;
    let gpu = app.gpu();
    let main_view = app.main_view();

    // pose inside fractal space
    let mut pose = Pose { p: Vec3 { x: 1.0,y: -2.0,z: 10.0, },o: Quat::ONE, };  // camera relative to fractal

    // ray marching parameters
    let mut march = March {
        pose: pose.into(),  // pose in fractal space
        scale: 1.0,  // rendering/lighting scale
        horizon: 64.32,  // maximum distance
        escape: 20.0,  // escape distance
        de_stop: 0.0001,  // "de_stop"
        de_stop_factor: 10.0,  // "de_stop_factor"
        max_steps: 500,  // maximum number of path trace steps
        max_iterations: 60,  // maximum number of iteractions
        tbd0: 0,
        view_dir: Vec4::UNIT_Z,  // viewing direction (mainly for measurement and lighting)
    };

    // rendering/lighting parameters
    let render = Render {
        albedo_color: Vec4 { x: 0.4,y: 0.5,z: 0.2,w: 1.0, },  // albedo
        key_light_pos: Vec4 { x: -10.0,y: 20.0,z: 30.0, w: 1.0, },  // key light position in fractal space
        key_light_color: Vec4 { x: 1.1,y: 1.2,z: 0.9, w: 1.0, },  // key light color
        shadow_power: Vec4 { x: 1.0,y: 1.2,z: 1.5, w: 40.0, },  // shadow RGB power (old film effect)
        sky_light_color: Vec4 { x: 0.16,y: 0.20,z: 0.28,w: 1.0, },  // sky light color
        ambient_light_color: Vec4 { x: 0.40,y: 0.28,z: 0.20,w: 1.0, },  // Walmart global illumination color
        background_color: Vec4 { x: 0.02,y: 0.05,z: 0.1,w: 0.5, },  // background color and fog multiplier
        glow_color: Vec4 { x: 0.1,y: 0.2,z: 0.2,w: 0.4, },  // proximity glow color and intensity
    };

    // the equirectangular image that the engine renders to and the projector shows
    let rgba_image = gpu.create_image2d::<u8>(ImageFormat::RGBA8SRGB,Vec2 { x: SIZE * 2,y: SIZE, },2,1,1,ImageUsage::SampledStorage,None)?;

    // start engine thread
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

    // create projector
    let mut projector = Projector::new(&app,&rgba_image)?;

    // create yardstick
    let mut yardstick = Yardstick::new(&app,&rgba_image,march,render)?;

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
                        projector.render(t)?;
                        app.end_frame(t,&[&main_view])?;
                    },
                    RenderHint::Drop(t) => {
                        app.begin_frame(t)?;
                        app.end_frame(t,&[&main_view])?;
                    },
                }

                if let AppState::Focused = app.state() {

                    // query distance to fractal
                    let depth = yardstick.measure_depth(projector.head_orientation());

                    // synchronize actions
                    app.sync_actions(&action_set)?;

                    // exit button
                    let exit = action_exit.get_bool()?;
                    if exit {
                        is_running = false;
                    }

                    // calculate new position from thumbstick
                    let nav = action_navigate.get_vec2()?;
                    if (nav.x != 0.0) || (nav.y != 0.0) {
                        // fly in viewing direction
                        let rotation = Mat3x3::<f32>::from(projector.head_orientation()).inv().transpose();
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