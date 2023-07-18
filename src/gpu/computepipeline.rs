use {
    crate::*,
    std::{
        rc::Rc,
        ptr::null_mut,
    },
};

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
