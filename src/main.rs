use {
    e::*,
    std::{
        rc::Rc,
        result::Result,
        fs::{
            File,
            read_to_string,
        },
        io::Read,
    },
};

mod mb3d;
use mb3d::*;

const KEY_ARROW_UP: u32 = 111;
const KEY_ARROW_DOWN: u32 = 116;
const KEY_ARROW_LEFT: u32 = 113;
const KEY_ARROW_RIGHT: u32 = 114;
const KEY_OBRACK: u32 = 34;
const KEY_CBRACK: u32 = 35;
const KEY_Q: u32 = 24;
const KEY_A: u32 = 38;
const KEY_W: u32 = 25;
const KEY_S: u32 = 39;
const KEY_E: u32 = 26;
const KEY_D: u32 = 40;
const KEY_R: u32 = 27;
const KEY_F: u32 = 41;
const KEY_ESC: u32 = 9;
const KEY_F1: u32 = 67;
const KEY_F2: u32 = 68;
const KEY_F3: u32 = 69;
const KEY_F4: u32 = 70;
const KEY_F5: u32 = 71;
const KEY_F6: u32 = 72;
const KEY_F7: u32 = 73;
const KEY_F8: u32 = 74;
const KEY_F9: u32 = 75;
const KEY_F10: u32 = 76;
const KEY_F11: u32 = 95;
const KEY_F12: u32 = 96;

const FOVY_DEG: f32 = 40.0;
const FORWARD_SENSITIVITY: f32 = 0.1;
const STRAFE_SENSITIVITY: f32 = 0.1;
const POINTER_SENSITIVITY: f32 = 0.001;

const SCALE_FACTOR: f32 = 1.01;
const DE_STOP_FACTOR: f32 = 1.01;
const ESCAPE_FACTOR: f32 = 0.1;

#[repr(u32)]
enum VisualizationMode {
    Output,
    Depth,
    Normal,
    DepthRB,
    IterationsRB,
    StepsRB,
    Occlusion,
}

#[repr(C)]
struct State {

    view: Mat4x4<f32>,            // view matrix

    size: Vec2<f32>,              // size of the output, in pixels
    fovy: f32,                    // vertical FoV
    scale: f32,                   // generic scale of the operation

    mode: VisualizationMode,      // visualization mode
    max_steps: u32,               // maximum number of ray marching steps
    max_iterations: u32,          // maximum number of iterations
    tbd0: u32,

    horizon: f32,                 // furthest distance to view
    escape: f32,                  // fractal iteration escape value
    de_stop: f32,                 // closest approach to the fractal
    focus: f32,                   // focus distance

    aperture: f32,                // aperture radius
    tbd1: f32,
    tbd2: f32,
    tbd3: f32,

    colors: [Vec4<f32>; 16],      // primary color table

    key_light_pos: Vec4<f32>,     // key light position

    key_light_color: Vec4<f32>,   // key light color

    shadow_power: Vec4<f32>,      // shadow power (a = sharpness)

    sky_light_color: Vec4<f32>,   // sky light color (a = fog strength)

    gi_light_color: Vec4<f32>,    // ambient light color

    background_color: Vec4<f32>,  // background color

    glow_color: Vec4<f32>,        // glow color (a = sharpness)
}

#[allow(dead_code)]
struct Rendering {
    image_view: ImageView,
    descriptor_set: DescriptorSet,
    command_buffer: CommandBuffer,
}

fn main() -> Result<(),String> {

    // MB3D decoding test, TODO: move to other project
    let mb3d_path = "states/julius/recombination.txt";
    let encoded = match read_to_string(mb3d_path) {
        Ok(data) => data,
        Err(error) => { return Err(error.to_string()) },
    };
    let mb3d = decode_mb3d(&encoded)?;
    dump_mb3d(&mb3d);

    let shader_path = "shaders/engine.spirv";
    let size = Vec2 { x: 1024i32,y: 1024i32, };

    let system = System::open()?;
    let frame = system.create_frame(Rect { o: Vec2 { x: 10i32,y: 10i32, },s: size, },"SDF Fractal Explorer",)?;
    let gpu = system.open_gpu()?;
    let mut surface = gpu.create_surface(&frame)?;

    let mut f = File::open(shader_path).expect("unable to open compute shader");
    let mut code = Vec::<u8>::new();
    f.read_to_end(&mut code).expect("unable to read compute shader");
    let compute_shader = Rc::new(gpu.create_compute_shader(&code)?);

    let descriptor_set_layout = gpu.create_descriptor_set_layout(&[DescriptorBinding::UniformBuffer,DescriptorBinding::StorageImage])?;
    let pipeline_layout = Rc::new(gpu.create_pipeline_layout(&[&descriptor_set_layout])?);
    let compute_pipeline = Rc::new(gpu.create_compute_pipeline(&pipeline_layout,&compute_shader)?);

    let mut pos = Vec3::<f32> { x: 0.0,y: 0.0,z: -30.0, };
    let mut dir = Quaternion::<f32>::ONE;
    let mut state = State {
        view: Mat4x4::<f32>::from_mv(Mat3x3::from(dir),pos),
        size: Vec2 { x: size.x as f32,y: size.y as f32, },
        fovy: 72.0.to_radians(),
        scale: 1.0,
        mode: VisualizationMode::Output,
        max_steps: 200,
        max_iterations: 60,
        tbd0: 0,
        horizon: 100.0,
        escape: 40.0,
        de_stop: 500.0,
        focus: 2.0,
        aperture: 0.01,
        tbd1: 0.0,
        tbd2: 0.0,
        tbd3: 0.0,
        colors: [
            Vec4 { x: 0.0,y: 0.0,z: 0.0, w: 1.0, },
            Vec4 { x: 0.0,y: 0.0,z: 0.2, w: 1.0, },
            Vec4 { x: 0.0,y: 0.2,z: 0.0, w: 1.0, },
            Vec4 { x: 0.0,y: 0.2,z: 0.2, w: 1.0, },
            Vec4 { x: 0.2,y: 0.0,z: 0.0, w: 1.0, },
            Vec4 { x: 0.2,y: 0.0,z: 0.2, w: 1.0, },
            Vec4 { x: 0.2,y: 0.2,z: 0.0, w: 1.0, },
            Vec4 { x: 0.2,y: 0.2,z: 0.2, w: 1.0, },
            Vec4 { x: 0.1,y: 0.1,z: 0.1, w: 1.0, },
            Vec4 { x: 0.1,y: 0.1,z: 0.3, w: 1.0, },
            Vec4 { x: 0.1,y: 0.3,z: 0.1, w: 1.0, },
            Vec4 { x: 0.1,y: 0.3,z: 0.3, w: 1.0, },
            Vec4 { x: 0.3,y: 0.1,z: 0.1, w: 1.0, },
            Vec4 { x: 0.3,y: 0.1,z: 0.3, w: 1.0, },
            Vec4 { x: 0.3,y: 0.3,z: 0.1, w: 1.0, },
            Vec4 { x: 0.3,y: 0.3,z: 0.3, w: 1.0, },
        ],
        key_light_pos: Vec4 { x: -20.0,y: -30.0,z: -10.0, w: 1.0, },  // somewhere above the origin
        key_light_color: Vec4 { x: 1.64,y: 1.47,z: 0.99, w: 1.0, },  // very bright yellow
        shadow_power: Vec4 { x: 1.0,y: 1.2,z: 1.5, w: 40.0, },  // shadow power (a = sharpness)
        sky_light_color: Vec4 { x: 0.16,y: 0.20,z: 0.28,w: 0.3, },   // sky light color (a = fog strength)
        gi_light_color: Vec4 { x: 0.40,y: 0.28,z: 0.20,w: 1.0, },    // ambient light color
        background_color: Vec4 { x: 0.0,y: 0.0,z: 0.01,w: 1.0, },  // background color
        glow_color: Vec4 { x: 0.2,y: 0.2,z: 0.2,w: 0.1, },        // glow color (a = power)
    };

    let uniform_buffer = gpu.create_uniform_buffer(&state)?;

    let fence = gpu.create_fence()?;
    let semaphore = gpu.create_semaphore()?;

    let mut delta: Vec2<f32> = Vec2::ZERO;
    let mut prev_position: Vec2<f32> = Vec2::ZERO;
    let mut button_pressed = false;

    let mut d_scale = 1.0;
    let mut d_de_stop = 1.0;
    let mut d_escape = 0.0;

    let mut renderings = Vec::<Rendering>::new();

    let mut close_clicked = false;
    while !close_clicked {

        let mut rebuild: Option<Vec2<usize>> = None;

        system.flush().into_iter().for_each(|(_,event)| {
            match event {
                Event::Close => {
                    close_clicked = true;
                },
                Event::Configure(r) => {
                    rebuild = Some(Vec2 { x: r.s.x as usize,y: r.s.y as usize, });
                },
                Event::Key(event) => {
                    match event {
                        KeyEvent::Press { code } => {
                            match code {
                                // forward/backward
                                KEY_ARROW_UP => {
                                    delta.y = FORWARD_SENSITIVITY * state.scale;
                                },
                                KEY_ARROW_DOWN => {
                                    delta.y = -FORWARD_SENSITIVITY * state.scale;
                                },

                                // strafing
                                KEY_ARROW_LEFT => {
                                    delta.x = -STRAFE_SENSITIVITY * state.scale;
                                },
                                KEY_ARROW_RIGHT => {
                                    delta.x = STRAFE_SENSITIVITY * state.scale;
                                },

                                // change mode
                                KEY_F1 => {
                                    state.mode = VisualizationMode::Output;
                                    println!("visualization mode: output");
                                },
                                KEY_F2 => {
                                    state.mode = VisualizationMode::Depth;
                                    println!("visualization mode: depth");
                                },
                                KEY_F3 => {
                                    state.mode = VisualizationMode::Normal;
                                    println!("visualization mode: normal");
                                },
                                KEY_F4 => {
                                    state.mode = VisualizationMode::DepthRB;
                                    println!("visualization mode: depth (colored)");
                                },
                                KEY_F5 => {
                                    state.mode = VisualizationMode::IterationsRB;
                                    println!("visualization mode: iterations");
                                },
                                KEY_F6 => {
                                    state.mode = VisualizationMode::StepsRB;
                                    println!("visualization mode: march steps");
                                },
                                KEY_F7 => {
                                    state.mode = VisualizationMode::Occlusion;
                                    println!("visualization mode: occlusion");
                                },

                                KEY_OBRACK => {
                                    d_scale = SCALE_FACTOR;
                                },
                                KEY_CBRACK => {
                                    d_scale = 1.0 / SCALE_FACTOR;
                                },

                                KEY_Q => {
                                    d_de_stop = DE_STOP_FACTOR;
                                },
                                KEY_A => {
                                    d_de_stop = 1.0 / DE_STOP_FACTOR;
                                },
                                KEY_W => {
                                    d_escape = ESCAPE_FACTOR;
                                },
                                KEY_S => {
                                    d_escape = -ESCAPE_FACTOR;
                                },

                                /*
                                KEY_E => {
                                    params_delta.z = DE_STOP_SENSITIVITY;
                                },
                                KEY_D => {
                                    params_delta.z = 1.0 / DE_STOP_SENSITIVITY;
                                },
                                KEY_R => {
                                    params_delta.w = FACTOR_SENSITIVITY;
                                },
                                KEY_F => {
                                    params_delta.w = 1.0 / FACTOR_SENSITIVITY;
                                }
                                */

                                _ => {
                                    println!("pressed {}",code);
                                },
                            }
                        },
                        KeyEvent::Release { code } => {
                            match code {

                                KEY_ESC => {
                                    close_clicked = true;
                                },

                                KEY_ARROW_UP | KEY_ARROW_DOWN => {
                                    delta.y = 0.0;
                                },
                                KEY_ARROW_LEFT | KEY_ARROW_RIGHT => {
                                    delta.x = 0.0;
                                },

                                KEY_F1 | KEY_F2 | KEY_F3 | KEY_F4 | KEY_F5 | KEY_F6 | KEY_F7 => { },

                                KEY_OBRACK | KEY_CBRACK => {
                                    d_scale = 1.0;
                                },

                                KEY_Q | KEY_A => {
                                    d_de_stop = 1.0;
                                },
                                KEY_W | KEY_S => {
                                    d_escape = 0.0;
                                },
                                /*
                                KEY_E | KEY_D => {
                                    params_delta.z = 1.0;
                                },
                                KEY_R | KEY_F => {
                                    params_delta.w = 1.0;
                                },
                                */

                                _ => {
                                    println!("released {}",code);
                                },
                            }
                        },
                    }
                },
                Event::Pointer(event) => {
                    match event {
                        PointerEvent::Down { position,button, } => {
                            if let Button::Left = button {
                                button_pressed = true;
                                prev_position = position;
                            }
                        },
                        PointerEvent::Up { position: _,button, } => {
                            if let Button::Left = button {
                                button_pressed = false;
                            }
                        },
                        PointerEvent::Move { position,.. } => {
                            if button_pressed {
                                let dp = position - prev_position;
                                dir *= Quaternion::<f32>::from_euler(-POINTER_SENSITIVITY * dp.y,POINTER_SENSITIVITY * dp.x,0.0);
                                prev_position = position;
                            }
                        },
                        _ => {
                            println!("pointer event: {}",event);
                        }
                    }
                },
                _ => { },
            }
        });

        // only process last configure event
        if let Some(s) = rebuild {
            state.size.x = s.x as f32;
            state.size.y = s.y as f32;
            let images = surface.reconfigure(s)?;
            renderings.clear();
            for image in images {
                let image_view = image.create_view()?;
                let descriptor_set = descriptor_set_layout.create_descriptor_set(&[&Descriptor::UniformBuffer(&uniform_buffer),&Descriptor::StorageImage(&image_view)])?;
                let command_buffer = gpu.create_command_buffer()?;
                command_buffer.begin()?;
                command_buffer.bind_compute_pipeline(&compute_pipeline);
                command_buffer.bind_descriptor_set(&pipeline_layout,0,&descriptor_set)?;
                command_buffer.dispatch(s.x,s.y,1);
                command_buffer.end()?;
                renderings.push(Rendering {
                    image_view,
                    descriptor_set,
                    command_buffer,
                });
            }
        }

        // process movement
        let rotation = Mat3x3::from(dir);
        let forward = rotation * Vec3::<f32> { x: 0.0,y: 0.0,z: 1.0, };
        let right = rotation * Vec3::<f32> { x: 1.0,y: 0.0,z: 0.0, };
        pos += delta.y * forward + delta.x * right;
        state.view = Mat4x4::<f32>::from_mv(rotation,pos);

        // process parameter updates
        state.scale = (state.scale * d_scale).clamp(0.00001,10.0);
        state.de_stop = (state.de_stop * d_de_stop).clamp(0.1,10000.0);
        state.escape = (state.escape + d_escape).clamp(1.0,100.0);

        // print which parameters got updated
        if d_scale != 1.0 {
            println!("scale: {}",state.scale);
        }
        if d_de_stop != 1.0 {
            println!("de_stop: {}",state.de_stop);
        }
        if d_escape != 0.0 {
            println!("escape: {}",state.escape);
        }

        // and update everything to the shaders
        uniform_buffer.update(&state);

        // only draw if a command buffer exists
        if renderings.len() > 0 {

            // acquire frame
            fence.reset()?;
            let index = surface.acquire(&fence)?;
            fence.wait()?;

            // render frame
            fence.reset()?;
            renderings[index].command_buffer.submit(None,Some(&semaphore),Some(&fence))?;
            fence.wait()?;

            // present frame
            if let Err(_) = surface.present(index,Some(&semaphore)) { }
        }
    }

    Ok(())
}
