use std::{
    rc::Rc,
    result::Result,
    fs::File,
    io::Read,
    time::Instant,
};

mod ffi {
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
//#![allow(dead_code)]
include!("ffi-reduced.rs");
}

mod base;
use base::*;

mod system;
use system::*;

mod gpu;
use gpu::*;

fn rebuild_command_buffers(
    surface: &Surface,
    command_buffers: &mut Vec<CommandBuffer>,
    compute_pipeline: &Rc<ComputePipeline>,
    pipeline_layout: &Rc<PipelineLayout>,
    size: Vec2<i32>,
) -> Result<(),String> {

    // create new descriptor sets
    let mut descriptor_sets: Vec<Rc<DescriptorSet>> = Vec::new();
    for i in 0..command_buffers.len() {
        let descriptor_set = Rc::new(pipeline_layout.create_descriptor_set()?);
        descriptor_set.update(0,Descriptor::StorageImage(surface.vk_image_views[i] as *mut u8));
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

fn main() -> Result<(),String> {

    let size = Vec2 { x: 1280i32,y: 768i32, };
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

    let pipeline_layout = Rc::new(gpu.create_pipeline_layout(&[DescriptorBinding::StorageImage])?);

    let mut f = File::open("shaders/engine.spirv").expect("unable to open compute shader");
    let mut code = Vec::<u8>::new();
    f.read_to_end(&mut code).expect("unable to read compute shader");
    let compute_shader = Rc::new(gpu.create_compute_shader(&code)?);

    let compute_pipeline = Rc::new(gpu.create_compute_pipeline(Rc::clone(&pipeline_layout),Rc::clone(&compute_shader))?);

    rebuild_command_buffers(&surface,&mut command_buffers,&compute_pipeline,&pipeline_layout,size)?;

    let acquired_semaphore = Rc::new(gpu.create_semaphore()?);
    let acquired_fence = Rc::new(gpu.create_fence()?);
    let submitted_fence = Rc::new(gpu.create_fence()?);
    let rendered_semaphore = Rc::new(gpu.create_semaphore()?);

    let mut close_clicked = false;
    while !close_clicked {

        let t_start = Instant::now();

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
            rebuild_command_buffers(&surface,&mut command_buffers,&compute_pipeline,&pipeline_layout,r.s)?;
        }

        let t_housekeeping = Instant::now() - t_start;

        acquired_fence.reset()?;
        let i = surface.acquire(Some(&acquired_semaphore),Some(&acquired_fence))?;
        acquired_fence.wait()?;

        let t_acquired = Instant::now() - t_start;

        submitted_fence.reset()?;
        gpu.submit_command_buffer(&command_buffers[i],Some(&acquired_semaphore),Some(&rendered_semaphore),Some(&submitted_fence))?;
        submitted_fence.wait()?;

        let t_submitted = Instant::now() - t_start;

        if let Err(_) = surface.present(i,Some(&rendered_semaphore)) { }

        let t_presented = Instant::now() - t_start;

        println!("hk:{:9} acq:{:9} subm:{:9} pres:{:9}",t_housekeeping.as_micros(),(t_acquired - t_housekeeping).as_micros(),(t_submitted - t_acquired).as_micros(),(t_presented - t_submitted).as_micros());
    }

    Ok(())
}
