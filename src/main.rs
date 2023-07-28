use std::{
    rc::Rc,
    result::Result,
    fs::{
        File,
        read_to_string,
    },
    io::Read,
    time::Instant,
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

fn random_color(start: Instant) -> Vec4<f32> {
    let now = (Instant::now() - start).as_nanos();
    let r = ((now & 255) as f32) / 255.0;
    let now = (Instant::now() - start).as_nanos();
    let g = ((now & 255) as f32) / 255.0;
    let now = (Instant::now() - start).as_nanos();
    let b = ((now & 255) as f32) / 255.0;
    Vec4 { x: r,y: g,z: b,w: 1.0, }
    //Vec4 { x: 0.0,y: 1.0,z: 0.5,w: 1.0, }
}

struct State {
    eye: Vec4<f32>, // vector from eye to top-left corner, in pixels
    world: Mat4x4<f32>, // world transformation
    background_color: Vec4<f32>,
}

fn main() -> Result<(),String> {

    let mb3d_path = "mb3d/mandelbulb_init.txt";
    let shader_path = "shaders/engine.spirv";

    let encoded = match read_to_string(mb3d_path) {
        Ok(data) => data,
        Err(error) => { return Err(error.to_string()) },
    };
    let mb3d = decode_mb3d(&encoded)?;
    dump_mb3d(&mb3d);

    let size = Vec2 { x: 800i32,y: 600i32, };
    let r = Rect { o: Vec2{ x: 0i32,y: 0i32, },s: size, };

    let system = Rc::new(System::open()?);
    let frame = Rc::new(system.create_frame(Rect { o: Vec2 { x: 10i32,y: 10i32, },s: size, },"Performance SDF",)?);
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

    let random_seed = Instant::now();

    let mut state = State {
        eye: Vec4 { x: 0.5 * size.x as f32, y: 0.5 * size.y as f32, z: -2000.0,w: 1.0, },
        world: Mat4x4::ONE,
        background_color: random_color(random_seed),
    };

    let uniform_buffer = Rc::new(gpu.create_uniform_buffer(&state)?);

    rebuild_command_buffers(&surface,&mut command_buffers,&compute_pipeline,&pipeline_layout,&uniform_buffer,size)?;
    
    let acquired_semaphore = Rc::new(gpu.create_semaphore()?);
    let acquired_fence = Rc::new(gpu.create_fence()?);
    let submitted_fence = Rc::new(gpu.create_fence()?);
    let rendered_semaphore = Rc::new(gpu.create_semaphore()?);

    let mut source_color = random_color(random_seed);
    let mut target_color = random_color(random_seed);
    let mut blend_tick = 100;

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
                _ => { },
            }
        });

        if let Some(r) = configure_event {
            surface.set_rect(&r)?;
            rebuild_command_buffers(&surface,&mut command_buffers,&compute_pipeline,&pipeline_layout,&uniform_buffer,r.s)?;
        }

        // Saul Goodman's Magic Animations
        blend_tick -= 1;
        if blend_tick <= 0 {
            source_color = target_color;
            target_color = random_color(random_seed);
            blend_tick = 100;
        }
        let f = (blend_tick as f32) * 0.01;
        state.background_color = f * source_color + (1.0 - f) * target_color;
        uniform_buffer.update(&state);

        acquired_fence.reset()?;
        let i = surface.acquire(Some(&acquired_semaphore),Some(&acquired_fence))?;
        acquired_fence.wait()?;

        submitted_fence.reset()?;
        gpu.submit_command_buffer(&command_buffers[i],Some(&acquired_semaphore),Some(&rendered_semaphore),Some(&submitted_fence))?;
        submitted_fence.wait()?;

        if let Err(_) = surface.present(i,Some(&rendered_semaphore)) { }
    }

    Ok(())
}
