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
enum Interlacing {
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

    size: Vec2<f32>,               // size of the output, in pixels
    fovy: f32,                     // vertical FoV
    scale: f32,                    // generic scale of the operation

    mode: VisualizationMode,       // visualization mode
    max_steps: u32,                // maximum number of ray marching steps
    max_iterations: u32,           // maximum number of iterations
    tbd0: u32,

    horizon: f32,                  // furthest distance to view
    escape: f32,                   // fractal iteration escape value
    de_stop: f32,                  // closest approach to the fractal
    focus: f32,                    // focus distance

    aperture: f32,                 // aperture radius
    tbd1: f32,
    tbd2: f32,
    tbd3: f32,

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
    interlacing: Interlacing,
    tbd0: u32,
    tbd1: u32,
    tbd2: u32,
}

enum RenderCommand {
    NewUniforms(Uniforms),
    NewImage(Arc<Image>,Vec2<usize>),
    Exit,
}

enum RenderState {
    Idle,
    Rendering(Interlacing,Vec2<usize>),
    Exiting,
}

struct Renderer<'a> {
    gpu: &'a Gpu,
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

impl<'a> Renderer<'a> {

    fn new(gpu: &'a Gpu,image: Arc<Image>,size: Vec2<usize>,initial_uniforms: Uniforms) -> Result<Renderer,String> {

        dprintln!("render_thread: getting queue");
        let queue = gpu.get_queue(1)?;

        dprintln!("render_thread: loading shader");
        let mut f = File::open("shaders/engine.spirv").expect("unable to open compute shader");
        let mut code = Vec::<u8>::new();
        f.read_to_end(&mut code).expect("unable to read compute shader");
        let compute_shader = gpu.create_compute_shader(&code)?;

        dprintln!("render_thread: creating descriptor set layout");
        let descriptor_set_layout = gpu.create_descriptor_set_layout(&[
            DescriptorBinding::UniformBuffer,
            DescriptorBinding::StorageImage,
        ])?;

        dprintln!("render_thread: creating pipeline layout");
        let pipeline_layout = gpu.create_pipeline_layout(&[&descriptor_set_layout],size_of::<PushConstants>())?;

        dprintln!("render_thread: creating compute pipeline");
        let compute_pipeline = gpu.create_compute_pipeline(&pipeline_layout,&compute_shader)?;

        dprintln!("render_thread: creating uniform buffer");
        let uniform_buffer = gpu.create_uniform_buffer(&initial_uniforms)?;

        dprintln!("render_thread: creating fence");
        let fence = gpu.create_fence()?;

        dprintln!("render_thread: create image view");
        let image_view = gpu.create_image_view(&image)?;

        dprintln!("render_thread: creating descriptor set");
        let descriptor_set = gpu.create_descriptor_set(&descriptor_set_layout,&[
            &Descriptor::UniformBuffer(&uniform_buffer),
            &Descriptor::StorageImage(&image_view),
        ])?;

        dprintln!("render_thread: creating command buffer");
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
            image,
            image_view,
            descriptor_set,
            command_buffer,
            size,
            state: RenderState::Idle,
        })
    }

    fn process_command(&mut self,command: RenderCommand) -> Result<(),String> {

        match command {

            RenderCommand::NewUniforms(new_uniforms) => {
                dprintln!("render_thread: new uniforms");
                self.uniforms = new_uniforms;
                self.uniform_buffer.update(&self.uniforms);
                self.state = RenderState::Rendering(Interlacing::Full16x16,Vec2 { x: self.size.x >> 4,y: self.size.y >> 4, });
            },

            RenderCommand::NewImage(image,size) => {
                dprintln!("render_thread: new image");
                self.image = image;
                self.size = size;
                self.image_view = self.gpu.create_image_view(&self.image)?;
                self.descriptor_set = self.gpu.create_descriptor_set(&self.descriptor_set_layout,&[
                    &Descriptor::UniformBuffer(&self.uniform_buffer),
                    &Descriptor::StorageImage(&self.image_view),
                ])?;
                self.uniforms.size = Vec2 { x: size.x as f32,y: size.y as f32, };
                self.uniform_buffer.update(&self.uniforms);
                self.state = RenderState::Rendering(Interlacing::Full16x16,Vec2 { x: self.size.x >> 4,y: self.size.y >> 4, });
            },

            RenderCommand::Exit => {
                self.state = RenderState::Exiting;
            }
        }

        Ok(())
    }

    fn render(&mut self) -> Result<(),String> {

        if let RenderState::Rendering(interlacing,size) = self.state {

            dprintln!("render thread: level {}, size {}",interlacing as usize,size);

            self.command_buffer = self.queue.create_command_buffer()?;

            let constants = PushConstants {
                interlacing,
                tbd0: 0,
                tbd1: 0,
                tbd2: 0,
            };
            self.command_buffer.begin()?;
            self.command_buffer.bind_compute_pipeline(&self.compute_pipeline);
            self.command_buffer.push_constants(&self.pipeline_layout,&constants);
            self.command_buffer.bind_descriptor_set(&self.pipeline_layout,0,&self.descriptor_set)?;
            self.command_buffer.dispatch(size.x,size.y,1);
            self.command_buffer.end()?;
    
            self.fence.reset()?;
            self.queue.submit(&self.command_buffer,None,None,Some(&self.fence))?;
            self.fence.wait()?;
    
            self.state = match interlacing {
                Interlacing::Full16x16 => RenderState::Rendering(Interlacing::Right8x16,Vec2 { x: self.size.x >> 4,y: self.size.y >> 4, }),
                Interlacing::Right8x16 => RenderState::Rendering(Interlacing::Bottom8x8,Vec2 { x: self.size.x >> 3,y: self.size.y >> 4, }),
                Interlacing::Bottom8x8 => RenderState::Rendering(Interlacing::Right4x8,Vec2 { x: self.size.x >> 3,y: self.size.y >> 3, }),
                Interlacing::Right4x8 => RenderState::Rendering(Interlacing::Bottom4x4,Vec2 { x: self.size.x >> 2,y: self.size.y >> 3, }),
                Interlacing::Bottom4x4 => RenderState::Rendering(Interlacing::Right2x4,Vec2 { x: self.size.x >> 2,y: self.size.y >> 2, }),
                Interlacing::Right2x4 => RenderState::Rendering(Interlacing::Bottom2x2,Vec2 { x: self.size.x >> 1,y: self.size.y >> 2, }),
                Interlacing::Bottom2x2 => RenderState::Rendering(Interlacing::Right1x2,Vec2 { x: self.size.x >> 1,y: self.size.y >> 1, }),
                Interlacing::Right1x2 => RenderState::Rendering(Interlacing::Bottom1x1,Vec2 { x: self.size.x,y: self.size.y >> 1, }),
                Interlacing::Bottom1x1 => RenderState::Idle,
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
        size: Vec2 { x: size.x as f32,y: size.y as f32, },
        fovy: 72.0.to_radians(),
        scale: 1.0,
        mode: VisualizationMode::Output,
        max_steps: 1000,
        max_iterations: 120,
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
            Vec4 { x: 0.3,y: 0.3,z: 0.3, w: 1.0, },
            Vec4 { x: 0.3,y: 0.3,z: 0.3, w: 1.0, },
            Vec4 { x: 0.3,y: 0.2,z: 0.2, w: 1.0, },
            Vec4 { x: 0.3,y: 0.1,z: 0.1, w: 1.0, },
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
        key_light_pos: Vec4 { x: -20.0,y: -30.0,z: -10.0, w: 1.0, },  // somewhere above the origin
        key_light_color: Vec4 { x: 1.64,y: 1.47,z: 0.99, w: 1.0, },  // very bright yellow
        shadow_power: Vec4 { x: 1.0,y: 1.2,z: 1.5, w: 40.0, },  // shadow power (a = sharpness)
        sky_light_color: Vec4 { x: 0.16,y: 0.20,z: 0.28,w: 0.8, },   // sky light color (a = fog strength)
        gi_light_color: Vec4 { x: 0.40,y: 0.28,z: 0.20,w: 1.0, },    // ambient light color
        background_color: Vec4 { x: 0.0,y: 0.0,z: 0.01,w: 1.0, },  // background color
        glow_color: Vec4 { x: 0.2,y: 0.2,z: 0.2,w: 0.1, },        // glow color (a = power)
    };

    let (tx,rx) = mpsc::channel();
    let render_gpu = Arc::clone(&gpu);
    let render_image = Arc::clone(&image);
    let render_size = size;
    let render_uniforms = uniforms.clone();
    let render_thread = thread::spawn(move || {
        if let Err(error) = {
            dprintln!("render_thread: creating renderer");
            let mut renderer = Renderer::new(&render_gpu,render_image,render_size,render_uniforms)?;
            dprintln!("render_thread: created renderer");
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

    dprintln!("creating fence");
    let fence = gpu.create_fence()?;

    dprintln!("creating semaphore");
    let semaphore = gpu.create_semaphore()?;

    let mut delta: Vec2<f32> = Vec2::ZERO;
    let mut prev_position: Vec2<f32> = Vec2::ZERO;
    let mut button_pressed = false;

    let mut d_scale = 1.0;
    let mut d_de_stop = 1.0;
    let mut d_escape = 0.0;

    let mut swapchain_images = Vec::<Image>::new();
    let mut command_buffers = Vec::<CommandBuffer>::new();

    let mut needs_rebuild = true;
    let mut is_running = true;
    while is_running {

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
                        // forward/backward
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
                            println!("visualization mode: output");
                        },
                        Key::F2 => {
                            uniforms.mode = VisualizationMode::Depth;
                            println!("visualization mode: depth");
                        },
                        Key::F3 => {
                            uniforms.mode = VisualizationMode::Normal;
                            println!("visualization mode: normal");
                        },
                        Key::F4 => {
                            uniforms.mode = VisualizationMode::DepthRB;
                            println!("visualization mode: depth (colored)");
                        },
                        Key::F5 => {
                            uniforms.mode = VisualizationMode::IterationsRB;
                            println!("visualization mode: iterations");
                        },
                        Key::F6 => {
                            uniforms.mode = VisualizationMode::StepsRB;
                            println!("visualization mode: march steps");
                        },
                        Key::F7 => {
                            uniforms.mode = VisualizationMode::Occlusion;
                            println!("visualization mode: occlusion");
                        },
                        Key::F8 => {
                            uniforms.mode = VisualizationMode::Debug;
                            println!("visualization mode: debug");
                        },

                        Key::OBracket => {
                            d_scale = SCALE_FACTOR;
                        },
                        Key::CBracket => {
                            d_scale = 1.0 / SCALE_FACTOR;
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

                        Key::F1 | Key::F2 | Key::F3 | Key::F4 | Key::F5 | Key::F6 | Key::F7 => { },

                        Key::OBracket | Key::CBracket => {
                            d_scale = 1.0;
                        },

                        Key::Q | Key::A => {
                            d_de_stop = 1.0;
                        },
                        Key::W | Key::S => {
                            d_escape = 0.0;
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

            dprintln!("waiting for GPU");
            gpu.wait_idle();

            dprintln!("creating image");
            image = Arc::new(gpu.create_image(size)?);

            dprintln!("sending new image to render thread");
            if let Err(error) = tx.send(RenderCommand::NewImage(Arc::clone(&image),size)) {
                dprintln!("unable to send new image to render thread ({})",error);
            }

            dprintln!("reconfiguring surface");
            swapchain_images = surface.reconfigure()?;

            dprintln!("rebuilding command buffers");
            command_buffers.clear();
            for i in 0..swapchain_images.len() {
                let command_buffer = blit_queue.create_command_buffer()?;
                command_buffer.begin()?;
                command_buffer.copy_image(&image,&swapchain_images[i],Rect { o: Vec2::<isize>::ZERO,s: Vec2 { x: size.x as isize,y: size.y as isize, }, });
                command_buffer.end()?;
                command_buffers.push(command_buffer);
            }

            dprintln!("rebuilt");

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
        uniforms.de_stop = (uniforms.de_stop * d_de_stop).clamp(0.1,10000.0);
        uniforms.escape = (uniforms.escape + d_escape).clamp(1.0,100.0);

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

        // instruct thread to start a new rendering
        if (delta.x != 0.0) || (delta.y != 0.0) || (d_scale != 1.0) || (d_de_stop != 1.0) || (d_escape != 0.0) || button_pressed {
            if let Err(error) = tx.send(RenderCommand::NewUniforms(uniforms.clone())) {
                dprintln!("unable to send uniforms to render thread ({})",error);
            }
        }

        // only draw if a command buffer exists
        if command_buffers.len() > 0 {
            fence.reset()?;
            let index = surface.acquire(&fence)?;
            fence.wait()?;
            fence.reset()?;
            blit_queue.submit(&command_buffers[index],None,Some(&semaphore),Some(&fence))?;
            fence.wait()?;
            if let Err(error) = surface.present(&blit_queue,index,Some(&semaphore)) {
                dprintln!("presentation error: {}",error);
                needs_rebuild = true;
            }
        }
    }

    dprintln!("done.");

    gpu.wait_idle();

    tx.send(RenderCommand::Exit).expect("unable to send exit to thread");
    render_thread.join().expect("unable to join with thread")?;

    Ok(())
}
