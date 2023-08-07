use std::{
    rc::Rc,
    result::Result,
    fs::{
        File,
        read_to_string,
    },
    io::Read,
};

mod ffi;

mod base;
use base::*;

mod system;
use system::*;

mod gpu;
use gpu::*;

mod mb3d;
use mb3d::*;

fn rebuild_command_buffers(
    surface: &Surface,
    command_buffers: &mut Vec<CommandBuffer>,
    compute_pipeline: &Rc<ComputePipeline>,
    pipeline_layout: &Rc<PipelineLayout>,
    uniform_buffer: &Rc<UniformBuffer>,
    size: Vec2<i32>,
) -> Result<(),String> {

    // create new descriptor sets
    let mut descriptor_sets: Vec<Rc<DescriptorSet>> = Vec::new();
    for i in 0..command_buffers.len() {
        let descriptor_set = Rc::new(pipeline_layout.create_descriptor_set()?);
        descriptor_set.update(0,Descriptor::UniformBuffer(uniform_buffer.vk_buffer as *mut u8,uniform_buffer.size as u64));
        descriptor_set.update(1,Descriptor::StorageImage(surface.vk_image_views[i] as *mut u8));
        descriptor_sets.push(descriptor_set);
    }

    // rebuild the command buffers
    for i in 0..command_buffers.len() {
        let cb = &mut command_buffers[i];
        cb.reset()?;
        cb.begin()?;
        cb.bind_compute_pipeline(&compute_pipeline);
        cb.bind_descriptor_set(&pipeline_layout,&descriptor_sets[i]);
        cb.dispatch(size.x as usize,size.y as usize,1);
        cb.end()?;
    }

    Ok(())
}

const KEY_ARROW_UP: u32 = 111;
const KEY_ARROW_DOWN: u32 = 116;
const KEY_ARROW_LEFT: u32 = 113;
const KEY_ARROW_RIGHT: u32 = 114;
const KEY_Q: u32 = 24;
const KEY_A: u32 = 38;
const KEY_W: u32 = 25;
const KEY_S: u32 = 39;
const KEY_E: u32 = 26;
const KEY_D: u32 = 40;
const KEY_R: u32 = 27;
const KEY_F: u32 = 41;

const FOVY_DEG: f32 = 40.0;
const FORWARD_SENSITIVITY: f32 = 0.1;
const STRAFE_SENSITIVITY: f32 = 0.1;
const POINTER_SENSITIVITY: f32 = 0.001;
const ITERATIONS_SENSITIVITY: f32 = 0.1;
const Z_STEP_SENSITIVITY: f32 = 0.01;
const DE_STOP_SENSITIVITY: f32 = 0.99;
const FACTOR_SENSITIVITY: f32 = 0.99;

struct State {
    view: Mat4x4<f32>,  // view transformation
    refs: Vec4<f32>,  // x: r.s.x as f32,y: r.s.y as f32,z: fovy,w: unused
    params: Vec4<f32>,  // x: max. iterations, y: z_step_div, z: initial de_stop, w: de_stop_factor
}

fn main() -> Result<(),String> {

    let mb3d_path = "states/julius/recombination.txt";
    let shader_path = "shaders/engine.spirv";

    let encoded = match read_to_string(mb3d_path) {
        Ok(data) => data,
        Err(error) => { return Err(error.to_string()) },
    };
    let mb3d = decode_mb3d(&encoded)?;
    dump_mb3d(&mb3d);

    let r = Rect { o: Vec2{ x: 0i32,y: 0i32, },s: Vec2 { x: 1024i32,y: 1024i32, }, };

    let system = Rc::new(System::open()?);
    let frame = Rc::new(system.create_frame(Rect { o: Vec2 { x: 10i32,y: 10i32, },s: r.s, },"Performance SDF",)?);
    let gpu = system.create_gpu()?;

    let mut surface = gpu.create_surface(Rc::clone(&frame),r)?;
    let count = surface.get_swapchain_count();

    let mut command_buffers: Vec<CommandBuffer> = Vec::new();
    for _ in 0..count {
        command_buffers.push(gpu.create_command_buffer()?);
    }

    let pipeline_layout = Rc::new(gpu.create_pipeline_layout(&[DescriptorBinding::UniformBuffer,DescriptorBinding::StorageImage])?);

    let mut f = File::open(shader_path).expect("unable to open compute shader");
    let mut code = Vec::<u8>::new();
    f.read_to_end(&mut code).expect("unable to read compute shader");
    let compute_shader = Rc::new(gpu.create_compute_shader(&code)?);

    let compute_pipeline = Rc::new(gpu.create_compute_pipeline(Rc::clone(&pipeline_layout),Rc::clone(&compute_shader))?);

    let mut pos = Vec3::<f32> { x: 0.0,y: 0.0,z: -50.0, };
    let mut dir = Quaternion::<f32>::ONE;
    let mut state = State {
        view: Mat4x4::<f32>::from_mv(Mat3x3::from(dir),pos),
        refs: Vec4::<f32> {
            x: r.s.x as f32,
            y: r.s.y as f32,
            z: FOVY_DEG.to_radians(),
            w: 0.0,
        },
        params: Vec4::<f32> {
            x: 30.0,
            y: 0.3,
            z: 0.0001,
            w: 1.0,
        },
    };

    let uniform_buffer = Rc::new(gpu.create_uniform_buffer(&state)?);

    rebuild_command_buffers(&surface,&mut command_buffers,&compute_pipeline,&pipeline_layout,&uniform_buffer,r.s)?;
    
    let acquired_semaphore = Rc::new(gpu.create_semaphore()?);
    let acquired_fence = Rc::new(gpu.create_fence()?);
    let submitted_fence = Rc::new(gpu.create_fence()?);
    let rendered_semaphore = Rc::new(gpu.create_semaphore()?);

    let mut delta: Vec2<f32> = Vec2::ZERO;
    let mut params_delta: Vec4<f32> = Vec4 { x: 0.0,y: 0.0,z: 1.0,w: 1.0, };
    let mut prev_position: Vec2<f32> = Vec2::ZERO;
    let mut left_pressed = false;
    let mut close_clicked = false;
    while !close_clicked {

        let mut configure_event: Option<Rect<i32>> = None;

        system.flush().into_iter().for_each(|(_,event)| {
            match event {
                Event::Close => {
                    close_clicked = true;
                },
                Event::Configure(r) => {
                    configure_event = Some(r.clone());
                },
                Event::Key(event) => {
                    match event {
                        KeyEvent::Press { code } => {
                            match code {
                                KEY_ARROW_UP => {
                                    delta.y = FORWARD_SENSITIVITY;
                                },
                                KEY_ARROW_DOWN => {
                                    delta.y = -FORWARD_SENSITIVITY;
                                },
                                KEY_ARROW_LEFT => {
                                    delta.x = -STRAFE_SENSITIVITY;
                                },
                                KEY_ARROW_RIGHT => {
                                    delta.x = STRAFE_SENSITIVITY;
                                },
                                KEY_Q => {
                                    params_delta.x = ITERATIONS_SENSITIVITY;
                                },
                                KEY_A => {
                                    params_delta.x = -ITERATIONS_SENSITIVITY;
                                },
                                KEY_W => {
                                    params_delta.y = Z_STEP_SENSITIVITY;
                                },
                                KEY_S => {
                                    params_delta.y = -Z_STEP_SENSITIVITY;
                                },
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
                                _ => {
                                    println!("pressed {}",code);
                                },
                            }
                        },
                        KeyEvent::Release { code } => {
                            match code {
                                KEY_ARROW_UP | KEY_ARROW_DOWN => {
                                    delta.y = 0.0;
                                },
                                KEY_ARROW_LEFT | KEY_ARROW_RIGHT => {
                                    delta.x = 0.0;
                                },
                                KEY_Q | KEY_A => {
                                    params_delta.x = 0.0;
                                },
                                KEY_W | KEY_S => {
                                    params_delta.y = 0.0;
                                },
                                KEY_E | KEY_D => {
                                    params_delta.z = 1.0;
                                },
                                KEY_R | KEY_F => {
                                    params_delta.w = 1.0;
                                }
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
                                left_pressed = true;
                                prev_position = position;
                            }
                        },
                        PointerEvent::Up { position: _,button, } => {
                            if let Button::Left = button {
                                left_pressed = false;
                            }
                        },
                        PointerEvent::Move { position,.. } => {
                            if left_pressed {
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
        if let Some(r) = configure_event {
            state.refs.x = r.s.x as f32;
            state.refs.y = r.s.y as f32;
            surface.set_rect(&r)?;
            rebuild_command_buffers(&surface,&mut command_buffers,&compute_pipeline,&pipeline_layout,&uniform_buffer,r.s)?;
        }

        // process movement
        let rotation = Mat3x3::from(dir);
        let forward = rotation * Vec3::<f32> { x: 0.0,y: 0.0,z: 1.0, };
        let right = rotation * Vec3::<f32> { x: 1.0,y: 0.0,z: 0.0, };
        pos += delta.y * forward + delta.x * right;
        state.view = Mat4x4::<f32>::from_mv(rotation,pos);

        // process parameter updates
        state.params.x = (state.params.x + params_delta.x).clamp(7.0,60.0);
        state.params.y = (state.params.y + params_delta.y).clamp(0.01,0.5);
        state.params.z = (state.params.z * params_delta.z).clamp(0.00001,0.1);
        state.params.w = (state.params.w * params_delta.w).clamp(0.00001,0.1);

        if params_delta.x != 0.0 {
            println!("max. iterations = {}",state.params.x);
        }
        if params_delta.y != 0.0 {
            println!("z_step_div = {}",state.params.y);
        }
        if params_delta.z != 1.0 {
            println!("de_stop = {}",state.params.z);
        }
        if params_delta.w != 1.0 {
            println!("de_stop_factor = {}",state.params.w);
        }

        // and update everything to the shaders
        uniform_buffer.update(&state);

        /*
        {
            let f = (0.5 * state.refs.z).tan();
            let aspect = state.refs.x / state.refs.y;
            let mx = aspect / f;
            let my = 1.0 / f;
            let x0 = -1.0 + 2.0 * (0.0 + 0.5) / state.refs.x;
            let x1 = -1.0 + 2.0 * ((r.s.x - 1) as f32 + 0.5) / state.refs.x;
            let y0 = -1.0 + 2.0 * (0.0 + 0.5) / state.refs.y;
            let y1 = -1.0 + 2.0 * ((r.s.y - 1) as f32 + 0.5) / state.refs.y;
            let screen0 = state.view * Vec4 { x: mx * x0,y: my * y0,z: 1.0,w: 1.0, };
            let screen1 = state.view * Vec4 { x: mx * x1,y: my * y1,z: 1.0,w: 1.0, };
            let origin = state.view * Vec4 { x: 0.0,y: 0.0,z: 0.0,w: 1.0, };
            println!("origin: {}, screen0: {}, screen1: {}",origin,screen0,screen1);
        }
        */

        // acquire frame
        acquired_fence.reset()?;
        let i = surface.acquire(Some(&acquired_semaphore),Some(&acquired_fence))?;
        acquired_fence.wait()?;

        // render frame
        submitted_fence.reset()?;
        gpu.submit_command_buffer(&command_buffers[i],Some(&acquired_semaphore),Some(&rendered_semaphore),Some(&submitted_fence))?;
        submitted_fence.wait()?;

        // present frame
        if let Err(_) = surface.present(i,Some(&rendered_semaphore)) { }
    }

    Ok(())
}
