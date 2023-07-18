use std::{
    rc::Rc,
    result::Result,
    fs::File,
    io::Read,
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

fn main() -> Result<(),String> {

    let size = Vec2 { x: 1280i32,y: 768i32, };

    let system = Rc::new(System::open()?);
    let frame = Rc::new(system.create_frame(Rect { o: Vec2 { x: 10i32,y: 10i32, },s: size, },"Raw Power",)?);

    let gpu = system.create_gpu()?;

    let r = Rect { o: Vec2{ x: 0i32,y: 0i32, },s: size, };
    let surface = gpu.create_surface(Rc::clone(&frame),r)?;
    
    let count = surface.get_swapchain_count();
    let mut command_buffers: Vec<CommandBuffer> = Vec::new();
    for _ in 0..count {
        command_buffers.push(gpu.create_command_buffer()?);
    }

    let pipeline_layout = Rc::new(gpu.create_pipeline_layout(&[DescriptorBinding::StorageImage])?);

    let mut descriptor_sets: Vec<Rc<DescriptorSet>> = Vec::new();
    for i in 0..count {
        let descriptor_set = Rc::new(pipeline_layout.create_descriptor_set()?);
        descriptor_set.update(0,Descriptor::StorageImage(surface.vk_image_views[i] as *mut u8));
        descriptor_sets.push(descriptor_set);
    }

    let mut f = File::open("shaders/engine.spirv").expect("unable to open compute shader");
    let mut code = Vec::<u8>::new();
    f.read_to_end(&mut code).expect("unable to read compute shader");
    let compute_shader = Rc::new(gpu.create_compute_shader(&code)?);

    let compute_pipeline = Rc::new(gpu.create_compute_pipeline(Rc::clone(&pipeline_layout),Rc::clone(&compute_shader))?);

    let semaphore = Rc::new(gpu.create_semaphore()?);
    let fence = Rc::new(gpu.create_fence()?);

    // prepare command buffers
    for index in 0..surface.get_swapchain_count() {
        let cb = &mut command_buffers[index];
        cb.begin()?;
        cb.bind_compute_pipeline(&compute_pipeline);
        cb.bind_descriptor_set(&pipeline_layout,&descriptor_sets[index]);
        cb.dispatch(size.x as usize,size.y as usize,1); // TODO: take current window size instead
        cb.end()?;
    }

    let mut close_clicked = false;
    while !close_clicked {

        // flush whatever X is doing
        system.flush().into_iter().for_each(|(_,event)| {
            println!("event {}",event);
            if let Event::Close = event {
                close_clicked = true;
            }
        });

        // acquire next frame
        let index = surface.acquire()?;

        // submit command buffer
        gpu.submit_command_buffer(&command_buffers[index],None,Some(&semaphore),Some(&fence))?;
        fence.wait()?;

        // present and flip
        surface.present(index,Some(&semaphore))?;
    }

    Ok(())
}
