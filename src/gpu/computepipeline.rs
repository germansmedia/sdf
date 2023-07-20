use {
    crate::*,
    std::{
        rc::Rc,
        ptr::null_mut,
        mem::MaybeUninit,
    },
};

impl Gpu {

    pub fn create_compute_pipeline(self: &Rc<Self>,pipeline_layout: Rc<PipelineLayout>,compute_shader: Rc<ComputeShader>) -> Result<ComputePipeline,String> {

        let info = ffi::VkComputePipelineCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            stage: ffi::VkPipelineShaderStageCreateInfo {
                sType: ffi::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                stage: ffi::VK_SHADER_STAGE_COMPUTE_BIT,
                module: compute_shader.vk_shader_module,
                pName: b"main\0".as_ptr() as *const i8,
                pSpecializationInfo: null_mut(),
            },
            layout: pipeline_layout.vk_pipeline_layout,
            basePipelineHandle: null_mut(),
            basePipelineIndex: -1,
        };
        let mut vk_pipeline = MaybeUninit::uninit();
        match unsafe { ffi::vkCreateComputePipelines(self.vk_device,null_mut(),1,&info,null_mut(),vk_pipeline.as_mut_ptr()) } {
            ffi::VK_SUCCESS => Ok(ComputePipeline {
                gpu: Rc::clone(&self),
                vk_pipeline: unsafe { vk_pipeline.assume_init() },
                compute_shader,
                pipeline_layout,
            }),
            code => Err(format!("Gpu::create_compute_pipeline: unable to create compute pipeline ({})",vk_code_to_string(code))),
        }
    }
}

pub struct ComputePipeline {
    pub gpu: Rc<Gpu>,
    pub vk_pipeline: ffi::VkPipeline,
    pub compute_shader: Rc<ComputeShader>,
    pub pipeline_layout: Rc<PipelineLayout>,
}

impl Drop for ComputePipeline {

    fn drop(&mut self) {
        unsafe { ffi::vkDestroyPipeline(self.gpu.vk_device,self.vk_pipeline,null_mut()) };
    }
}
