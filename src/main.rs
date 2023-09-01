use {
    e::*,
    std::{
        result::Result,
        fs::File,
        io::Read,
        sync::{
            mpsc,
            Arc,
        },
        mem::size_of,
        thread,
    },
};

const FORWARD_SENSITIVITY: f32 = 0.1;
const STRAFE_SENSITIVITY: f32 = 0.1;
const POINTER_SENSITIVITY: f32 = 0.001;

const SCALE_FACTOR: f32 = 1.01;
const DE_STOP_FACTOR: f32 = 1.01;
const ESCAPE_FACTOR: f32 = 0.1;
const FOCUS_FACTOR: f32 = 0.001;
const APERTURE_FACTOR: f32 = 0.001;
const MH04ZSD_FACTOR: f32 = 1.01;

const DESMOND9_FULL16X16: (u32,u32) = (0,0);
const DESMOND9_RIGHT8X16: (u32,u32) = (8,8);
const DESMOND9_BOTTOM8X8: [(u32,u32); 2] = [(8,0),(0,8),];
const DESMOND9_RIGHT4X8: [(u32,u32); 4] = [(4,4),(12,12),(12,4),(4,12),];
const DESMOND9_BOTTOM4X4: [(u32,u32); 8] = [(4,0),(12,8),(12,0),(4,8),(0,4),(8,12),(8,4),(0,12),];
const DESMOND9_RIGHT2X4: [(u32,u32); 16] = [
    (2,2),(10,10),(10,2),(2,10),(6,6),(14,14),(14,6),(6,14),(6,2),(14,10),(14,2),(6,10),(2,6),(10,14),(10,6),(2,14),
];
const DESMOND9_BOTTOM2X2: [(u32,u32); 32] = [
    (2,0),(10,8),(10,0),(2,8), (6,4),(14,12),(14,4),(6,12),(6,0),(14,8), (14,0),(6,8), (2,4),(10,12),(10,4),(2,12),
    (0,2),(8,10),(8,2), (0,10),(4,6),(12,14),(12,6),(4,14),(4,2),(12,10),(12,2),(4,10),(0,6),(8,14), (8,6), (0,14),
];
const DESMOND9_RIGHT1X2: [(u32,u32); 64] = [
    (1,1),(9,9),  (9,1), (1,9), (5,5),(13,13),(13,5),(5,13),(5,1),(13,9), (13,1),(5,9), (1,5),(9,13), (9,5), (1,13),
    (3,3),(11,11),(11,3),(3,11),(7,7),(15,15),(15,7),(7,15),(7,3),(15,11),(15,3),(7,11),(3,7),(11,15),(11,7),(3,15),
    (3,1),(11,9), (11,1),(3,9), (7,5),(15,13),(15,5),(7,13),(7,1),(15,9), (15,1),(7,9), (3,5),(11,13),(11,5),(3,13),
    (1,3),(9,11), (9,3), (1,11),(5,7),(13,15),(13,7),(5,15),(5,3),(13,11),(13,3),(5,11),(1,7),(9,15), (9,7), (1,15),
];
const DESMOND9_BOTTOM1X1: [(u32,u32); 128] = [
    (1,0),(9,8),  (9,0), (1,8), (5,4),(13,12),(13,4),(5,12), (5,0),(13,8), (13,0),(5,8), (1,4),(9,12), (9,4), (1,12),
    (3,2),(11,10),(11,2),(3,10),(7,6),(15,14),(15,6),(7,14), (7,2),(15,10),(15,2),(7,10),(3,6),(11,14),(11,6),(3,14),
    (3,0),(11,8), (11,0),(3,8), (7,4),(15,12),(15,4),(7,12), (7,0),(15,8), (15,0),(7,8), (3,4),(11,12),(11,4),(3,12),
    (1,2),(9,10), (9,2), (1,10),(5,6),(13,14),(13,6),(5,14), (5,2),(13,10),(13,2),(5,10),(1,6),(9,14), (9,6), (1,14),
    (0,1),(8,9),  (8,1), (0,9), (4,5),(12,13),(12,5),(4,13), (4,1),(12,9), (12,1),(4,9), (0,5),(8,13), (8,5), (0,13),
    (2,3),(10,11),(10,3),(2,11),(6,7),(14,15),(14,7),(6,15), (6,3),(14,11),(14,3),(6,11),(2,7),(10,15),(10,7),(2,15),
    (2,1),(10,9), (10,1),(2,9), (6,5),(14,13),(14,5),(6,13), (6,1),(14,9), (14,1),(6,9), (2,5),(10,13),(10,5),(2,13),
    (0,3),(8,11), (8,3),(0,11), (4,7),(12,15),(12,7),(4,15), (4,3),(12,11),(12,3),(4,11),(0,7),(8,15), (8,7), (0,15),
];

#[derive(Clone)]
#[repr(u32)]
enum VisualizationMode {
    Output,
    Depth,
    Normal,
    DepthRB,
    IterationsRB,
    StepsRB,
    Occlusion,
    Debug,
}

#[derive(Clone,Copy)]
#[repr(u32)]
enum Progressive {
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

// State shared between rust and compute shader
#[derive(Clone)]
#[repr(C)]
struct Uniforms {

    view: Mat4x4<f32>,             // view matrix

    fovy: f32,                     // vertical FoV
    scale: f32,                    // generic scale of the operation
    focus: f32,                    // focus distance
    aperture: f32,                 // aperture radius

    mode: VisualizationMode,       // visualization mode
    max_steps: u32,                // maximum number of ray marching steps
    max_iterations: u32,           // maximum number of iterations
    tbd0: u32,

    horizon: f32,                  // furthest distance to view
    escape: f32,                   // fractal iteration escape value
    de_stop: f32,                  // closest approach to the fractal
    step_mul: f32,                 // multiplier to prevent undersampling

    de_stop_factor: f32,           // from mb3d
    mh04zsd: f32,                  // from mb3d
    tbd1: f32,
    tbd2: f32,

    colors: [Vec4<f32>; 16],       // primary color table

    key_light_pos: Vec4<f32>,      // key light position

    key_light_color: Vec4<f32>,    // key light color

    shadow_power: Vec4<f32>,       // shadow power (a = sharpness)

    sky_light_color: Vec4<f32>,    // sky light color (a = fog strength)

    gi_light_color: Vec4<f32>,     // ambient light color

    background_color: Vec4<f32>,   // background color

    glow_color: Vec4<f32>,         // glow color (a = sharpness)
}

#[repr(C)]
#[derive(Clone,Copy)]
struct PushConstants {
    size: Vec2<f32>,
    progressive: Progressive,
    offset: u32,
}

enum RenderCommand {
    NewUniforms(Uniforms),
    NewImage(Arc<Image>,Vec2<usize>),
    Exit,
}

enum RenderState {
    Idle,
    Rendering(Progressive,usize),
    Exiting,
}

struct Renderer {
    gpu: Arc<Gpu>,
    queue: Queue,
    descriptor_set_layout: DescriptorSetLayout,
    pipeline_layout: PipelineLayout,
    compute_shader: ComputeShader,
    compute_pipeline: ComputePipeline,
    uniforms: Uniforms,
    uniform_buffer: UniformBuffer,
    fence: Fence,
    image: Arc<Image>,
    image_view: ImageView,
    descriptor_set: DescriptorSet,
    command_buffer: CommandBuffer,
    size: Vec2<usize>,
    state: RenderState,
}

impl Renderer {

    fn new(gpu: Arc<Gpu>,initial_image: Arc<Image>,initial_size: Vec2<usize>,initial_uniforms: Uniforms) -> Result<Renderer,String> {

        // request render queue
        let queue = gpu.get_queue(1)?;

        // load compute shader
        let mut f = File::open("shaders/engine.spirv").expect("unable to open compute shader");
        let mut code = Vec::<u8>::new();
        f.read_to_end(&mut code).expect("unable to read compute shader");
        let compute_shader = gpu.create_compute_shader(&code)?;

        // prepare pipeline
        let descriptor_set_layout = gpu.create_descriptor_set_layout(&[
            DescriptorBinding::UniformBuffer,
            DescriptorBinding::StorageImage,
        ])?;
        let pipeline_layout = gpu.create_pipeline_layout(&[&descriptor_set_layout],size_of::<PushConstants>())?;
        let compute_pipeline = gpu.create_compute_pipeline(&pipeline_layout,&compute_shader)?;

        // prepare uniform buffer
        let uniform_buffer = gpu.create_uniform_buffer(&initial_uniforms)?;

        // prepare initial image view
        let image_view = gpu.create_image_view(&initial_image)?;

        // prepare descriptor set
        let descriptor_set = gpu.create_descriptor_set(&descriptor_set_layout,&[
            &Descriptor::UniformBuffer(&uniform_buffer),
            &Descriptor::StorageImage(&image_view),
        ])?;

        // fence and command buffer
        let fence = gpu.create_fence()?;
        let command_buffer = queue.create_command_buffer()?;

        Ok(Renderer {
            gpu,
            queue,
            descriptor_set_layout,
            pipeline_layout,
            compute_shader,
            compute_pipeline,
            uniforms: initial_uniforms,
            uniform_buffer,
            fence,
            image: initial_image,
            image_view,
            descriptor_set,
            command_buffer,
            size: initial_size,
            state: RenderState::Idle,
        })
    }

    fn process_command(&mut self,command: RenderCommand) -> Result<(),String> {

        match command {

            // receive new uniforms from main thread
            RenderCommand::NewUniforms(new_uniforms) => {

                // update uniforms
                self.uniforms = new_uniforms;
                self.uniform_buffer.update(&self.uniforms);

                // restart progressive rendering
                self.state = RenderState::Rendering(Progressive::Full16x16,0);
            },

            // receive new image and configuration size
            RenderCommand::NewImage(image,size) => {

                // update image and size
                self.image = image;
                self.size = size;

                // recreate image view for this new image (drops old one)
                self.image_view = self.gpu.create_image_view(&self.image)?;

                // recreate descriptor set (drops old one)
                self.descriptor_set = self.gpu.create_descriptor_set(&self.descriptor_set_layout,&[
                    &Descriptor::UniformBuffer(&self.uniform_buffer),
                    &Descriptor::StorageImage(&self.image_view),
                ])?;

                // restart progressive rendering
                self.state = RenderState::Rendering(Progressive::Full16x16,0);
            },

            // laterr...
            RenderCommand::Exit => {
                self.state = RenderState::Exiting;
            }
        }

        Ok(())
    }

    fn render(&mut self) -> Result<(),String> {

        // if we're rendering
        if let RenderState::Rendering(progressive,pass) = self.state {

            // create new command buffer (drops old one)
            self.command_buffer = self.queue.create_command_buffer()?;

            // prepare pixel offset inside 16x16 tile
            let offset = match progressive {
                Progressive::Full16x16 => DESMOND9_FULL16X16,
                Progressive::Right8x16 => DESMOND9_RIGHT8X16,
                Progressive::Bottom8x8 => DESMOND9_BOTTOM8X8[pass],
                Progressive::Right4x8 => DESMOND9_RIGHT4X8[pass],
                Progressive::Bottom4x4 => DESMOND9_BOTTOM4X4[pass],
                Progressive::Right2x4 => DESMOND9_RIGHT2X4[pass],
                Progressive::Bottom2x2 => DESMOND9_BOTTOM2X2[pass],
                Progressive::Right1x2 => DESMOND9_RIGHT1X2[pass],
                Progressive::Bottom1x1 => DESMOND9_BOTTOM1X1[pass],
            };

            // prepare constants for pushing via command buffer
            let constants = PushConstants {
                size: Vec2 { x: self.size.x as f32,y: self.size.y as f32, },
                progressive,
                offset: (offset.1 << 4) | offset.0,
            };

            // prepare command buffer
            self.command_buffer.begin()?;
            self.command_buffer.bind_compute_pipeline(&self.compute_pipeline);
            self.command_buffer.push_constants(&self.pipeline_layout,&constants);
            self.command_buffer.bind_descriptor_set(&self.pipeline_layout,0,&self.descriptor_set)?;
            self.command_buffer.dispatch(self.size.x >> 4,self.size.y >> 4,1);
            self.command_buffer.end()?;
    
            // render
            self.fence.reset()?;
            self.queue.submit(&self.command_buffer,None,None,Some(&self.fence))?;
            self.fence.wait()?;
    
            // go to next progressive state
            self.state = match progressive {
                Progressive::Full16x16 => RenderState::Rendering(Progressive::Right8x16,0),
                Progressive::Right8x16 => RenderState::Rendering(Progressive::Bottom8x8,0),
                Progressive::Bottom8x8 => if pass >= 1 {
                    RenderState::Rendering(Progressive::Right4x8,0)
                }
                else {
                    RenderState::Rendering(Progressive::Bottom8x8,pass + 1)
                },
                Progressive::Right4x8 => if pass >= 3 {
                    RenderState::Rendering(Progressive::Bottom4x4,0)
                }
                else {
                    RenderState::Rendering(Progressive::Right4x8,pass + 1)
                },
                Progressive::Bottom4x4 => if pass >= 7 {
                    RenderState::Rendering(Progressive::Right2x4,0)
                }
                else {
                    RenderState::Rendering(Progressive::Bottom4x4,pass + 1)
                },
                Progressive::Right2x4 => if pass >= 15 {
                    RenderState::Rendering(Progressive::Bottom2x2,0)
                }
                else {
                    RenderState::Rendering(Progressive::Right2x4,pass + 1)
                },
                Progressive::Bottom2x2 => if pass >= 31 {
                    RenderState::Rendering(Progressive::Right1x2,0)
                }
                else {
                    RenderState::Rendering(Progressive::Bottom2x2,pass + 1)
                },
                Progressive::Right1x2 => if pass >= 63 {
                    RenderState::Rendering(Progressive::Bottom1x1,0)
                }
                else {
                    RenderState::Rendering(Progressive::Right1x2,pass + 1)
                },
                Progressive::Bottom1x1 => if pass >= 127 {
                    RenderState::Idle
                }
                else {
                    RenderState::Rendering(Progressive::Bottom1x1,pass + 1)
                },
            };
        }

        Ok(())
    }
}

fn main() -> Result<(),String> {

    let mut size = Vec2 { x: 512usize,y: 512usize, };

    let mut system = System::open()?;
    let frame = system.create_frame(
        Rect {
            o: Vec2 { x: 10i32,y: 10i32, },
            s: Vec2 { x: size.x as i32,y: size.y as i32, },
        },
        "Fractal Explorer",
    )?;
    let gpu = Arc::new(system.open_gpu(2)?);
    let blit_queue = gpu.get_queue(0)?;
    let mut surface = gpu.create_surface(&frame)?;
    let mut image = Arc::new(gpu.create_image(size)?);

    let mut pos = Vec3::<f32> { x: 0.0,y: 0.0,z: -30.0, };
    let mut dir = Quaternion::<f32>::ONE;
    let mut uniforms = Uniforms {
        view: Mat4x4::<f32>::from_mv(Mat3x3::from(dir),pos),
        fovy: 45.0.to_radians(),
        scale: 1.0,
        focus: 2.0,
        aperture: 0.01,
        mode: VisualizationMode::Output,
        max_steps: 1000,
        max_iterations: 60,
        tbd0: 0,
        horizon: 50.0,
        escape: 20.0,
        de_stop: 0.0001,
        step_mul: 1.0,
        de_stop_factor: 10.0,
        mh04zsd: 0.1,
        tbd1: 0.0,
        tbd2: 0.0,
        colors: [
            Vec4 { x: 0.2,y: 0.2,z: 0.3, w: 1.0, },
            Vec4 { x: 0.1,y: 0.1,z: 0.3, w: 1.0, },
            Vec4 { x: 0.1,y: 0.0,z: 0.2, w: 1.0, },
            Vec4 { x: 0.2,y: 0.0,z: 0.1, w: 1.0, },
            Vec4 { x: 0.3,y: 0.0,z: 0.0, w: 1.0, },
            Vec4 { x: 0.3,y: 0.1,z: 0.0, w: 1.0, },
            Vec4 { x: 0.3,y: 0.2,z: 0.0, w: 1.0, },
            Vec4 { x: 0.3,y: 0.3,z: 0.0, w: 1.0, },
            Vec4 { x: 0.2,y: 0.3,z: 0.0, w: 1.0, },
            Vec4 { x: 0.1,y: 0.3,z: 0.0, w: 1.0, },
            Vec4 { x: 0.0,y: 0.3,z: 0.0, w: 1.0, },
            Vec4 { x: 0.0,y: 0.3,z: 0.1, w: 1.0, },
            Vec4 { x: 0.0,y: 0.2,z: 0.2, w: 1.0, },
            Vec4 { x: 0.0,y: 0.1,z: 0.3, w: 1.0, },
            Vec4 { x: 0.0,y: 0.0,z: 0.3, w: 1.0, },
            Vec4 { x: 0.0,y: 0.0,z: 0.2, w: 1.0, },
        ],
        key_light_pos: Vec4 { x: -10.0,y: -20.0,z: -40.0, w: 1.0, },  // somewhere above the origin
        key_light_color: Vec4 { x: 1.64,y: 1.47,z: 0.99, w: 1.0, },  // very bright yellow
        shadow_power: Vec4 { x: 1.0,y: 1.2,z: 1.5, w: 40.0, },  // shadow power (a = sharpness)
        sky_light_color: Vec4 { x: 0.16,y: 0.20,z: 0.28,w: 1.0, },   // sky light color (a = fog strength)
        gi_light_color: Vec4 { x: 0.40,y: 0.28,z: 0.20,w: 1.0, },    // ambient light color
        background_color: Vec4 { x: 0.0,y: 0.1,z: 0.2,w: 1.0, },  // background color
        glow_color: Vec4 { x: 0.2,y: 0.2,z: 0.2,w: 0.4, },        // glow color (a = power)
    };

    let (tx,rx) = mpsc::channel();
    let render_gpu = Arc::clone(&gpu);
    let render_image = Arc::clone(&image);
    let render_size = size;
    let render_uniforms = uniforms.clone();
    let render_thread = thread::spawn(move || {
        if let Err(error) = {
            let mut renderer = Renderer::new(render_gpu,render_image,render_size,render_uniforms)?;
            loop {
                match renderer.state {
                    RenderState::Idle => {
                        if let Ok(command) = rx.recv() {
                            renderer.process_command(command)?;
                        }
                        else {
                            dprintln!("render_thread: command receive error");
                            break;
                        }
                    },
                    RenderState::Rendering(_,_) => {
                        for command in rx.try_iter() {
                            renderer.process_command(command)?;
                        }
                    },
                    RenderState::Exiting => {
                        break;
                    },
                }
                renderer.render()?;
            }
            Result::<(),String>::Ok(())
        } {
            dprintln!("render_thread: crashed ({})",error);
        }
        Result::<(),String>::Ok(())
    });

    let fence = gpu.create_fence()?;
    let semaphore = gpu.create_semaphore()?;

    let mut swapchain_images = Vec::<Image>::new();
    let mut command_buffers = Vec::<CommandBuffer>::new();

    let mut delta: Vec2<f32> = Vec2::ZERO;
    let mut prev_position: Vec2<f32> = Vec2::ZERO;
    let mut button_pressed = false;

    let mut d_scale = 1.0;
    let mut d_de_stop = 1.0;
    let mut d_escape = 0.0;
    let mut d_focus = 0.0;
    let mut d_aperture = 0.0;
    let mut d_de_stop_factor = 1.0;
    let mut d_mh04zsd = 1.0;

    let mut needs_rebuild = true;
    let mut is_running = true;
    while is_running {

        let mut mode_change = false;
        system.flush().into_iter().for_each(|(_,event)| {
            match event {
                Event::Close => {
                    is_running = false;
                },
                Event::Configure(r) => {
                    let new_size = Vec2::<usize> { x: r.s.x as usize,y: r.s.y as usize, };
                    if new_size != size {
                        size = new_size;
                        needs_rebuild = true;
                    }
                },
                Event::KeyPress(key) => {
                    match key {
                        Key::Up => {
                            delta.y = FORWARD_SENSITIVITY * uniforms.scale;
                        },
                        Key::Down => {
                            delta.y = -FORWARD_SENSITIVITY * uniforms.scale;
                        },

                        // strafing
                        Key::Left => {
                            delta.x = -STRAFE_SENSITIVITY * uniforms.scale;
                        },
                        Key::Right => {
                            delta.x = STRAFE_SENSITIVITY * uniforms.scale;
                        },

                        // change mode
                        Key::F1 => {
                            uniforms.mode = VisualizationMode::Output;
                            mode_change = true;
                            println!("visualization mode: output");
                        },
                        Key::F2 => {
                            uniforms.mode = VisualizationMode::Depth;
                            mode_change = true;
                            println!("visualization mode: depth");
                        },
                        Key::F3 => {
                            uniforms.mode = VisualizationMode::Normal;
                            mode_change = true;
                            println!("visualization mode: normal");
                        },
                        Key::F4 => {
                            uniforms.mode = VisualizationMode::DepthRB;
                            mode_change = true;
                            println!("visualization mode: depth (colored)");
                        },
                        Key::F5 => {
                            uniforms.mode = VisualizationMode::IterationsRB;
                            mode_change = true;
                            println!("visualization mode: iterations");
                        },
                        Key::F6 => {
                            uniforms.mode = VisualizationMode::StepsRB;
                            mode_change = true;
                            println!("visualization mode: march steps");
                        },
                        Key::F7 => {
                            uniforms.mode = VisualizationMode::Occlusion;
                            mode_change = true;
                            println!("visualization mode: occlusion");
                        },
                        Key::F8 => {
                            uniforms.mode = VisualizationMode::Debug;
                            mode_change = true;
                            println!("visualization mode: debug");
                        },

                        Key::OBracket => {
                            d_scale = 1.0 / SCALE_FACTOR;
                        },
                        Key::CBracket => {
                            d_scale = SCALE_FACTOR;
                        },

                        Key::Q => {
                            d_de_stop = DE_STOP_FACTOR;
                        },
                        Key::A => {
                            d_de_stop = 1.0 / DE_STOP_FACTOR;
                        },
                        Key::W => {
                            d_escape = ESCAPE_FACTOR;
                        },
                        Key::S => {
                            d_escape = -ESCAPE_FACTOR;
                        },
                        Key::E => {
                            d_focus = FOCUS_FACTOR;
                        },
                        Key::D => {
                            d_focus = -FOCUS_FACTOR;
                        },
                        Key::R => {
                            d_aperture = APERTURE_FACTOR;
                        },
                        Key::F => {
                            d_aperture = -APERTURE_FACTOR;
                        },
                        Key::T => {
                            d_de_stop_factor = DE_STOP_FACTOR;
                        },
                        Key::G => {
                            d_de_stop_factor = 1.0 / DE_STOP_FACTOR;
                        },
                        Key::Y => {
                            d_mh04zsd = MH04ZSD_FACTOR;
                        },
                        Key::H => {
                            d_mh04zsd = 1.0 / MH04ZSD_FACTOR;
                        },

                        _ => {
                            println!("pressed {}",key);
                        },
                    }
                },
                Event::KeyRelease(key) => {
                    match key {
                        Key::Escape => {
                            is_running = false;
                        },

                        Key::Up | Key::Down => {
                            delta.y = 0.0;
                        },
                        Key::Left | Key::Right => {
                            delta.x = 0.0;
                        },

                        Key::F1 | Key::F2 | Key::F3 | Key::F4 | Key::F5 | Key::F6 | Key::F7 | Key::F8 => { },

                        Key::OBracket | Key::CBracket => {
                            d_scale = 1.0;
                        },

                        Key::Q | Key::A => {
                            d_de_stop = 1.0;
                        },
                        Key::W | Key::S => {
                            d_escape = 0.0;
                        },
                        Key::E | Key::D => {
                            d_focus = 0.0;
                        },
                        Key::R | Key::F => {
                            d_aperture = 0.0;
                        },
                        Key::T | Key::G => {
                            d_de_stop_factor = 1.0;
                        },
                        Key::Y | Key::H => {
                            d_mh04zsd = 1.0;
                        },

                        _ => {
                            println!("released {}",key);
                        },
                    }
                },
                Event::MousePress(pos,button) => {
                    if let Button::Left = button {
                        button_pressed = true;
                        prev_position = Vec2 { x: pos.x as f32,y: pos.y as f32, };
                    }
                },
                Event::MouseRelease(_,button) => {
                    if let Button::Left = button {
                        button_pressed = false;
                    }
                },
                Event::MouseMove(pos) =>  {
                    if button_pressed {
                        let fpos = Vec2 { x: pos.x as f32,y: pos.y as f32, };
                        let dp = fpos - prev_position;
                        dir *= Quaternion::<f32>::from_euler(-POINTER_SENSITIVITY * dp.y,POINTER_SENSITIVITY * dp.x,0.0);
                        prev_position = fpos;
                    }
                }
                _ => { },
            }
        });

        // rebuild image if needed
        if needs_rebuild {

            gpu.wait_idle();

            image = Arc::new(gpu.create_image(size)?);
            if let Err(error) = tx.send(RenderCommand::NewImage(Arc::clone(&image),size)) {
                dprintln!("unable to send new image to render thread ({})",error);
            }

            swapchain_images = surface.reconfigure()?;

            command_buffers.clear();
            for i in 0..swapchain_images.len() {
                let command_buffer = blit_queue.create_command_buffer()?;
                command_buffer.begin()?;
                command_buffer.copy_image(&image,&swapchain_images[i],Rect { o: Vec2::<isize>::ZERO,s: Vec2 { x: size.x as isize,y: size.y as isize, }, });
                command_buffer.end()?;
                command_buffers.push(command_buffer);
            }

            needs_rebuild = false;
        }

        // process movement
        let rotation = Mat3x3::from(dir);
        let forward = rotation * Vec3::<f32> { x: 0.0,y: 0.0,z: 1.0, };
        let right = rotation * Vec3::<f32> { x: 1.0,y: 0.0,z: 0.0, };
        pos += delta.y * forward + delta.x * right;
        uniforms.view = Mat4x4::<f32>::from_mv(rotation,pos);

        // process parameter updates
        uniforms.scale = (uniforms.scale * d_scale).clamp(0.00001,10.0);
        uniforms.de_stop = (uniforms.de_stop * d_de_stop).clamp(0.00001,10.0);
        uniforms.escape = (uniforms.escape + d_escape).clamp(1.0,100.0);
        uniforms.focus = (uniforms.focus + d_focus).clamp(0.0,10.0);
        uniforms.aperture = (uniforms.aperture + d_aperture).clamp(0.0,1.0);
        uniforms.de_stop_factor = (uniforms.de_stop_factor * d_de_stop_factor).clamp(0.01,1000.0);
        uniforms.mh04zsd = (uniforms.mh04zsd * d_mh04zsd).clamp(0.0001,100.0);

        // print which parameters got updated
        if d_scale != 1.0 {
            println!("scale: {}",uniforms.scale);
        }
        if d_de_stop != 1.0 {
            println!("de_stop: {}",uniforms.de_stop);
        }
        if d_escape != 0.0 {
            println!("escape: {}",uniforms.escape);
        }
        if d_focus != 0.0 {
            println!("focus: {}",uniforms.focus);
        }
        if d_aperture != 0.0 {
            println!("aperture: {}",uniforms.aperture);
        }
        if d_de_stop_factor != 1.0 {
            println!("de_stop_factor: {}",uniforms.de_stop_factor);
        }
        if d_mh04zsd != 1.0 {
            println!("mh04zsd: {}",uniforms.mh04zsd);
        }

        // instruct thread to start a new rendering
        if (delta.x != 0.0) ||
            (delta.y != 0.0) ||
            (d_scale != 1.0) ||
            (d_de_stop != 1.0) ||
            (d_escape != 0.0) ||
            (d_focus != 0.0) ||
            (d_aperture != 0.0) ||
            (d_de_stop_factor != 1.0) ||
            (d_mh04zsd != 1.0) ||
            mode_change ||
            button_pressed {
            if let Err(error) = tx.send(RenderCommand::NewUniforms(uniforms.clone())) {
                dprintln!("unable to send uniforms to render thread ({})",error);
            }
        }

        // only draw if a command buffer exists
        if command_buffers.len() > 0 {
            fence.reset()?;
            match surface.acquire(&fence) {
                Err(error) => {
                    dprintln!("error acquiring frame: {}",error);
                    needs_rebuild = true;
                },
                Ok(index) => {
                    fence.wait()?;
                    fence.reset()?;
                    blit_queue.submit(&command_buffers[index],None,Some(&semaphore),Some(&fence))?;
                    fence.wait()?;
                    if let Err(error) = surface.present(&blit_queue,index,Some(&semaphore)) {
                        dprintln!("presentation error: {}",error);
                        needs_rebuild = true;
                    }        
                },
            };
        }
    }

    dprintln!("done.");

    gpu.wait_idle();

    tx.send(RenderCommand::Exit).expect("unable to send exit to thread");
    render_thread.join().expect("unable to join with thread")?;

    Ok(())
}
