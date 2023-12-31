// SDF - render fractals in VR
// by Desmond Germans, 2023

use {
    std::{
        result::Result,
        sync::Arc,
    },
    crate::*,
};

#[derive(Clone,Copy,Debug)]
#[repr(C)]
struct Storage {
    depth: f32,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct Config {
    pub size: Vec2<u32>,
    pub tbd0: u32,
    pub tbd1: u32,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct Uniforms {
    pub config: Config,
    pub params: Params,
}

pub struct Yardstick {
    _app: Arc<App>,
    _gpu: Arc<Gpu>,
    queue: Arc<Queue>,
    uniforms: Uniforms,
    uniform_buffer: Arc<UniformBuffer<Uniforms>>,
    storage: Storage,
    storage_buffer: Arc<StorageBuffer<Storage>>,
    _pipeline_layout: Arc<PipelineLayout>,
    _descriptor_set_layout: Arc<DescriptorSetLayout>,
    _measure_pipeline: Arc<ComputePipeline>,
    _descriptor_set: Arc<DescriptorSet>,
    _measure_shader: Arc<ComputeShader>,
    command_buffer: Arc<CommandBuffer>,
}

impl Yardstick {

    pub fn new(app: &Arc<App>,rgba_image: &Arc<Image2D>,params: Params) -> Result<Yardstick,String> {

        let gpu = app.gpu();
        let queue = gpu.queue(0)?;
        let descriptor_set_layout = gpu.build_descriptor_set_layout()
            .uniform_buffer()
            .storage_buffer()
            .build()?;
        let pipeline_layout = gpu.create_pipeline_layout(&[&descriptor_set_layout],0)?;
        let size = rgba_image.size();
        let uniforms = Uniforms {
            config: Config {
                size: size.into(),
                tbd0: 0,
                tbd1: 0,
            },
            params,
        };
        let uniform_buffer = gpu.create_uniform_buffer(&queue,AccessStyle::Shared,&[uniforms])?;
        let storage = Storage {
            depth: 0.0,
        };
        let storage_buffer = gpu.create_storage_buffer(&queue,AccessStyle::Shared,&[storage])?;
        let descriptor_set = descriptor_set_layout.build_descriptor_set()?
            .uniform_buffer(&uniform_buffer)
            .storage_buffer(&storage_buffer)
            .build();
        let code = app.load_asset("assets","yardstick.spirv")?;
        let measure_shader = gpu.create_compute_shader(&code)?;
        let measure_pipeline = pipeline_layout.create_compute_pipeline(&measure_shader)?;
        let command_buffer = queue.create_command_buffer()?;
        command_buffer.begin()?;
        command_buffer.bind_compute_pipeline(&measure_pipeline);
        command_buffer.bind_compute_descriptor_set(&pipeline_layout,0,&descriptor_set);
        command_buffer.dispatch(1,1,1);
        command_buffer.wait_dispatch_write(&storage_buffer);
        command_buffer.end()?;
        Ok(Yardstick {
            _app: Arc::clone(&app),
            _gpu: gpu,
            queue,
            _descriptor_set_layout: descriptor_set_layout,
            uniforms,
            storage,
            _pipeline_layout: pipeline_layout,
            uniform_buffer,
            storage_buffer,
            _descriptor_set: descriptor_set,
            _measure_shader: measure_shader,
            _measure_pipeline: measure_pipeline,
            command_buffer,
        })
    }

    pub fn measure_depth(&mut self,params: &Params) -> Result<f32,String> {
        self.uniforms.params = *params;
        self.uniform_buffer.write(&self.queue)?[0] = self.uniforms;
        self.queue.submit(&self.command_buffer,None,None)?;
        self.queue.wait()?;
        self.storage = self.storage_buffer.read(&self.queue)?[0];
        Ok(self.storage.depth)
    }
}
