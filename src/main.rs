// SDF - render fractals in VR
// by Desmond Germans, 2023

use {
    std::{
        result::Result,
        sync::{
            Arc,
            mpsc,
        },
        thread::spawn,
        time::{
            SystemTime,
            UNIX_EPOCH,
        },
        fs::read_to_string,
    },
    e_macros::*,
    e_base::*,
    e_gpu::*,
    e_hal::*,
    e_codec_image::*,
};

mod base;
use base::*;

mod viewer;
mod tiler;
mod projector;
mod yardstick;

const FOV_ANGLE: f32 = 0.5;

const TILE_SIZE: Vec2<u32> = Vec2 { x: 64,y: 64, };
const TILE_COUNT: Vec2<u32> = Vec2 { x: 16,y: 9, };

const FORWARD_SPEED: f32 = 0.02;
const STRAFE_SPEED: f32 = 0.01;

// eye-candy reference from coolors.com
const PALETTES: [[u32; 5]; 16] = [
    [0xc05746,0xadc698,0xd0e3c4,0xffffff,0x503047],
    [0x0b0033,0x370031,0x832232,0xce8964,0xeaf27c],
    [0xaba9bf,0xbeb7df,0xd4f2d2,0x34113f,0x868784],
    [0x1e1e24,0x92140c,0xfff8f0,0xffcf99,0x111d4a],
    [0x1ac8ed,0xaed4e6,0xaf7595,0x8c2155,0x5c1a1b],
    [0x18206f,0x17255a,0xf5e2c8,0xd88373,0xbd1e1e],
    [0x0a210f,0x14591d,0x99aa38,0xe1e289,0xacd2ed],
    [0x202030,0x39304a,0x635c51,0x7d7461,0xb0a990],
    [0x95f9e3,0x69ebd0,0x49d49d,0x558564,0x564946],
    [0x495867,0x577399,0xbdd5ea,0xf7f7ff,0xfe5f55],
    [0xf7f7f2,0xe4e6c3,0x899878,0x222725,0x121113],
    [0x33658a,0x86bbd8,0x2f4858,0xf6ae2d,0xf26419],
    [0xf1dac4,0xa69cac,0x474973,0x161b33,0x0d0c1d],
    [0xf7f052,0xf28123,0xd34e24,0x563f1b,0x38726c],
    [0x2589bd,0x187795,0x38686a,0xa3b4a2,0xcdc6ae],
    [0xd0cfec,0x6a8e7f,0x989572,0xc6ae82,0xedc7cf],
];

fn random() -> u32 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() >> 3
}

fn hex_to_vec(hex: u32) -> Vec4<f32> {
    let r = ((hex >> 16) as f32) / 255.0;
    let g = (((hex >> 8) & 255) as f32) / 255.0;
    let b = ((hex & 255) as f32) / 255.0;
    Vec4 { x: r,y: g,z: b,w: 1.0, }
}

fn generate_params(
    pose: Pose<f32>,
    scale: f32,
    horizon: f32,
    escape: f32,
    dtf_const: f32,
    dtf_linear: f32,
    max_steps: usize,
    max_iterations: usize,
    step_size: f32,
    iod: f32,
) -> Params {
    let n = (random() & 15) as usize;
    logd!("n = {}",n);
    let palette = &PALETTES[n];

    let (mut background_color,palette) = match random() & 3 {
        0 => (hex_to_vec(palette[0]),[
            hex_to_vec(palette[1]),
            hex_to_vec(palette[2]),
            hex_to_vec(palette[3]),
            hex_to_vec(palette[4]),
        ]),
        1 => (hex_to_vec(palette[0]),[
            hex_to_vec(palette[4]),
            hex_to_vec(palette[3]),
            hex_to_vec(palette[2]),
            hex_to_vec(palette[1]),
        ]),
        2 => (hex_to_vec(palette[4]),[
            hex_to_vec(palette[0]),
            hex_to_vec(palette[1]),
            hex_to_vec(palette[2]),
            hex_to_vec(palette[3]),
        ]),
        3 => (hex_to_vec(palette[4]),[
            hex_to_vec(palette[3]),
            hex_to_vec(palette[2]),
            hex_to_vec(palette[1]),
            hex_to_vec(palette[0]),
        ]),
        _ => panic!("huh... interesting..."),
    };
    background_color.w = 0.4 * (((random() & 31) as f32) / 31.0);
    Params {
        pose: pose.into(),
        forward_dir: Vec4::UNIT_Z,
        key_light_pos: Vec4 { x: -20.0,y: 30.0,z: 30.0, w: 1.0, },
        key_light_color: Vec4 { x: 0.9,y: 0.8,z: 0.7,w: 1.0, },
        shadow_power: Vec4 { x: 1.0,y: 1.2,z: 1.5, w: 40.0, },
        sky_light_color: Vec4 { x: 0.16,y: 0.20,z: 0.28,w: 1.0, },
        ambient_light_color: Vec4 { x: 0.40,y: 0.28,z: 0.20,w: 1.0, },
        background_color,
        glow_color: Vec4 { x: 0.1,y: 0.2,z: 0.2,w: 0.4, },
        palette: [
            palette[0],
            palette[1],
            palette[2],
            palette[3],
            palette[0],
            palette[1],
            palette[2],
            palette[3],
        ],
        scale,
        horizon,
        escape,
        dtf_const,
        dtf_linear,
        max_steps: max_steps as u32,
        max_iterations: max_iterations as u32,
        step_size,
        iod,
        tbd0: 0,
        tbd1: 0,
        tbd2: 0,
    }
}

app!(sdf2);
fn main() -> Result<(),String> {

    let app = App::new(3)?;
    let gpu = app.gpu();
    let main_view = app.main_view();

    // pose inside fractal space
    let mut pose = Pose { p: Vec3 { x: 0.0,y: 0.0,z: 10.0, },o: Quat::ONE, };  // camera relative to fractal

    // parameters
    let mut view_size = 512usize;
    let mut horizon = 100.0f32;
    let mut escape = 20.0f32;
    let mut dtf_const = 1.0f32;
    let mut dtf_linear = 0.0f32;
    let mut max_steps = 600usize;
    let mut max_iterations = 40usize;
    let mut step_size = 1.0f32;
    let mut iod = 0.03f32;
    read_to_string("params.txt").unwrap().lines().for_each(|line| {
        let v: Vec<&str> = line.split(':').collect();
        if v.len() == 2 {
            logd!("read \"{}\" : \"{}\"",v[0].trim(),v[1].trim());
            match v[0].trim().to_lowercase().as_str() {
                "view_size" => { view_size = v[1].trim().parse().unwrap(); },
                "horizon" => { horizon = v[1].trim().parse().unwrap(); },
                "escape" => { escape = v[1].trim().parse().unwrap(); },
                "dtf_const" => { dtf_const = v[1].trim().parse().unwrap(); },
                "dtf_linear" => { dtf_linear = v[1].trim().parse().unwrap(); },
                "max_steps" => { max_steps = v[1].trim().parse().unwrap(); },
                "max_iterations" => { max_iterations = v[1].trim().parse().unwrap(); },
                "step_size" => { step_size = v[1].trim().parse().unwrap(); },
                "iod" => { iod = v[1].trim().parse().unwrap(); },
                _ => { },
            }
        }
    });
    let mut params = generate_params(pose,1.0,horizon,escape,dtf_const,dtf_linear,max_steps,max_iterations,step_size,iod);

    // keyframes
    let mut keyframes = [
        params,
        params,
    ];

    // the equirectangular image that the viewer renders to and the projector shows
    let rgba_image = gpu.create_empty_image2d(ImageFormat::RGBA8SRGB,Vec2 { x: view_size * 2,y: view_size, },2,1,1,ImageUsage::SampledStorage,AccessStyle::Gpu)?;

    // start viewer thread
    let viewer_gpu = Arc::clone(&gpu);
    let depth_occlusion_code = app.load_asset("assets","viewer_depth_occlusion.spirv")?;
    let lighting_code = app.load_asset("assets","viewer_lighting.spirv")?;
    let viewer_rgba_image = Arc::clone(&rgba_image);
    let (viewer_tx,viewer_rx) = mpsc::channel();
    let viewer_thread = spawn(move || {
        let mut viewer = viewer::Viewer::new(
            &viewer_gpu,
            &depth_occlusion_code,
            &lighting_code,
            &viewer_rgba_image,
            params,
        )?;
        match {
            loop {
                match viewer.state {
                    viewer::State::Idle => {
                        if let Ok(command) = viewer_rx.recv() {
                            viewer.process_command(command)?;
                        }
                        else {
                            loge!("command receive error");
                            break;
                        }
                    },
                    viewer::State::Rendering(_,_,_) => {
                        for command in viewer_rx.try_iter() {
                            viewer.process_command(command)?;
                        }
                    },
                    viewer::State::Exiting => {
                        break;
                    },
                }
                viewer.render()?;
            }
            Result::<(),String>::Ok(())
        } {
            Ok(()) => { },
            Err(error) => { loge!("viewer thread crashed ({})",error); },
        }
        Result::<(),String>::Ok(())
    });

    // start tiler thread
    let tiler_gpu = Arc::clone(&gpu);
    let tiler_code = app.load_asset("assets","tiler.spirv")?;
    let (tiler_tx,tiler_rx) = mpsc::channel();
    let tiler_thread = spawn(move || {
        let mut tiler = tiler::Tiler::new(
            &tiler_gpu,
            &tiler_code,
            tiler::Config {
                type_: tiler::Type::Quad,
                flags: tiler::TILER_FLAGS_STEREO,
                tile_size: TILE_SIZE,
                tile_count: TILE_COUNT,
                current_tile: Vec2::ZERO,
                fovs: [
                    Fov { l: -FOV_ANGLE,r: FOV_ANGLE,b: -FOV_ANGLE,t: FOV_ANGLE, },
                    Fov { l: -FOV_ANGLE,r: FOV_ANGLE,b: -FOV_ANGLE,t: FOV_ANGLE, },
                ],
            },
            params,
        )?;
        match {
            loop {
                match tiler.state {
                    tiler::State::Idle => {
                        if let Ok(command) = tiler_rx.recv() {
                            tiler.process_command(command)?;
                        }
                        else {
                            loge!("command receive error");
                            break;
                        }
                    },
                    tiler::State::RenderingPhoto | tiler::State::RenderingVideo => {
                        for command in tiler_rx.try_iter() {
                            tiler.process_command(command)?;
                        }
                    },
                    tiler::State::Exiting => {
                        break;
                    },
                }
                tiler.render()?;
            }
            Result::<(),String>::Ok(())
        } {
            Ok(()) => { },
            Err(error) => { loge!("tiler thread crashed ({})",error); },
        }
        Result::<(),String>::Ok(())
    });

    // create projector
    let mut projector = projector::Projector::new(&app,&rgba_image)?;

    // create yardstick
    let mut yardstick = yardstick::Yardstick::new(&app,&rgba_image,params)?;

    // prepare XR actions
    let action_set = app.create_action_set("action_set")?;
    let action_photo = action_set.create_float_action("photo","/user/hand/right/input/trigger/value")?;
    let action_exit = action_set.create_bool_action("exit","/user/hand/right/input/b/click")?;
    let action_next = action_set.create_bool_action("next","/user/hand/right/input/a/click")?;
    let action_navigate = action_set.create_vec2_action("navigate","/user/hand/right/input/thumbstick")?;
    let action_params = action_set.create_vec2_action("params","/user/hand/left/input/thumbstick")?;
    app.attach_action_set(&action_set)?;

    // and go...
    let mut is_running = true;
    let mut photo_pressed = false;
    let mut next_pressed = false;
    let mut first_recorded = false;
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

                    // synchronize actions
                    app.sync_actions(&action_set)?;

                    // exit button
                    let exit = action_exit.get_bool()?;
                    if exit {
                        is_running = false;
                    }

                    // generate new fractal
                    let next = action_next.get_bool()?;
                    if next && !next_pressed {
                        params = generate_params(pose,params.scale,horizon,escape,dtf_const,dtf_linear,max_steps,max_iterations,step_size,iod);
                        viewer_tx.send(viewer::Command::Update(params)).unwrap();
                    }
                    next_pressed = next;

                    // take clip
                    let photo = action_photo.get_float()? > 0.8;
                    if photo && !photo_pressed {
                        let rotation = Mat4x4::<f32>::from(projector.head_orientation());
                        if !first_recorded {
                            keyframes[0] = params;
                            keyframes[0].pose *= rotation;
                            first_recorded = true;
                            logd!("from here...");
                        }
                        else {
                            keyframes[1] = params;
                            keyframes[1].pose *= rotation;
                            logd!("...to here");
                            tiler_tx.send(
                                tiler::Command::Video(
                                    tiler::Config {
                                        type_: tiler::Type::Quad,
                                        flags: 0,
                                        tile_size: TILE_SIZE,
                                        tile_count: TILE_COUNT,
                                        current_tile: Vec2::ZERO,
                                        fovs: [
                                            Fov { l: -FOV_ANGLE,r: FOV_ANGLE,b: -FOV_ANGLE,t: FOV_ANGLE, },
                                            Fov { l: -FOV_ANGLE,r: FOV_ANGLE,b: -FOV_ANGLE,t: FOV_ANGLE, },        
                                        ],
                                    },
                                    keyframes[0],
                                    keyframes[1],
                                )
                            ).unwrap();
                            first_recorded = false;
                        }

                        /*
                        let mut photo_params = params;
                        photo_params.pose *= rotation;
                        tiler_tx.send(tiler::Command::Execute(
                            tiler::Config {
                                type_: tiler::Type::Quad,
                                flags: tiler::TILER_FLAGS_STEREO,
                                tile_size: TILE_SIZE,
                                tile_count: TILE_COUNT,
                                current_tile: Vec2::ZERO,
                                fovs: [
                                    Fov { l: -FOV_ANGLE,r: FOV_ANGLE,b: -FOV_ANGLE,t: FOV_ANGLE, },
                                    Fov { l: -FOV_ANGLE,r: FOV_ANGLE,b: -FOV_ANGLE,t: FOV_ANGLE, },
                                ],
                            },
                            photo_params,
                        )).unwrap();
                        */
                    }
                    photo_pressed = photo;

                    // adjust parameters
                    let control = action_params.get_vec2()?;
                    if (control.x != 0.0) {
                        params.dtf_const *= 1.1f32.powf(control.x);
                        params.scale = yardstick.measure_depth(&params)?;
                        viewer_tx.send(viewer::Command::Update(params)).unwrap();
                        logd!("r = {}, dtf_const = {}",params.scale,params.dtf_const);
                    }

                    // calculate new position from thumbstick
                    let nav = action_navigate.get_vec2()?;
                    if (nav.x != 0.0) || (nav.y != 0.0) {

                        // get fly/view direction
                        let rotation = Mat3x3::<f32>::from(projector.head_orientation()).inv().transpose();
                        let forward = rotation * Vec3::<f32>::UNIT_Z;
                        let right = rotation * Vec3::<f32>::UNIT_X;

                        // measure distance and adjust scale factor
                        //params.pose = pose.into();
                        params.forward_dir = Vec4 { x: -forward.x,y: -forward.y,z: -forward.z,w: 0.0, };
                        params.scale = yardstick.measure_depth(&params)?;
                        if params.scale > 2.0 {
                            params.scale = 2.0;
                        }
                        if params.scale < 0.00001 {
                            params.scale = 0.00001;
                        }

                        // fly in viewing direction
                        pose.p += -FORWARD_SPEED * params.scale * nav.y * forward + STRAFE_SPEED * params.scale * nav.x * right;
                        params.pose = pose.into();
                        viewer_tx.send(viewer::Command::Update(params)).unwrap();
                    }
                }
            },
        }
    }

    let _ = viewer_tx.send(viewer::Command::Exit);
    let _ = viewer_thread.join();
    let _ = tiler_tx.send(tiler::Command::Exit);
    let _ = tiler_thread.join();

    gpu.wait_idle();

    Ok(())
}