// SDF - render fractals in VR
// by Desmond Germans, 2023

use {
    crate::*,
    std::{
        result::Result,
        sync::Arc,
        mem::size_of,
    },
};

#[derive(Clone,Copy,Debug)]
#[repr(C)]
struct Storage {
    depth: f32,
}

pub struct Yardstick {
    _app: Arc<App>,
    _gpu: Arc<Gpu>,
    queue: Arc<Queue>,
    uniforms: EngineUniforms,
    uniform_buffer: Arc<UniformBuffer>,
    storage: Storage,
    storage_buffer: Arc<StorageBuffer>,
    _pipeline_layout: Arc<PipelineLayout>,
    _descriptor_set_layout: Arc<DescriptorSetLayout>,
    _measure_pipeline: Arc<ComputePipeline>,
    _descriptor_set: Arc<DescriptorSet>,
    _measure_shader: Arc<ComputeShader>,
    command_buffer: Arc<CommandBuffer>,
}

impl Yardstick {

    pub fn new(app: &Arc<App>,rgba_image: &Arc<Image2D>,march: March,render: Render,) -> Result<Yardstick,String> {

        let gpu = app.gpu();
        let queue = gpu.queue(0)?;
        let descriptor_set_layout = gpu.build_descriptor_set_layout()
            .uniform_buffer()
            .storage_buffer()
            .build()?;
        let pipeline_layout = gpu.create_pipeline_layout(&[&descriptor_set_layout],size_of::<Push>())?;
        let size = rgba_image.size();
        let uniforms = EngineUniforms {
            view: ViewConfig {
                width: size.x as u32,
                height: size.y as u32,
                type_: ViewType::StereoEquirect,
                tbd0: 0,
                fov: Fov { l: 0.0,r: 0.0,b: 0.0,t: 0.0, },
            },
            progress: Progress {
                phase: Phase::Full16x16,
                offset: Vec2::ZERO,
                tbd0: 0,
            },
            march,
            render,
        };
        let uniform_buffer = gpu.create_uniform_buffer(Init::Data(&[uniforms]))?;
        let storage = Storage {
            depth: 0.0,
        };
        let storage_buffer = gpu.create_storage_buffer(Init::Data(&[storage]))?;
        let descriptor_set = descriptor_set_layout.build_descriptor_set()?
            .uniform_buffer(&uniform_buffer)
            .storage_buffer(&storage_buffer)
            .build();
        let code = app.load_asset("assets","measure_cs.spirv")?;
        let measure_shader = gpu.create_compute_shader(&code)?;
        let measure_pipeline = pipeline_layout.create_compute_pipeline(&measure_shader)?;
        let command_buffer = queue.create_command_buffer()?;
        command_buffer.begin()?;
        command_buffer.bind_compute_pipeline(&measure_pipeline);
        command_buffer.bind_compute_descriptor_set(&pipeline_layout,0,&descriptor_set);
        command_buffer.push_constants(&pipeline_layout,&Push { eye: 0,face: 0,anisotropy: EquirectAnisotropy::Square,y_offset: 0, });
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

    pub fn measure_depth(&mut self,orientation: Quat<f32>) -> Result<f32,String> {
        let rotation = Mat3x3::<f32>::from(orientation).inv().transpose();
        let forward = rotation * Vec3::<f32>::UNIT_Z;
        self.uniforms.march.view_dir = Vec4 { x: forward.x,y: forward.y,z: forward.z,w: 0.0, };
        self.uniform_buffer.data_mut()?[0] = self.uniforms;
        self.queue.submit(&self.command_buffer,None,None)?;
        self.queue.wait()?;
        self.storage = self.storage_buffer.data()?[0];
        logd!("and this actually works??  {}",self.storage.depth);
        Ok(self.storage.depth)
    }
}
