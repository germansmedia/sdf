use {
    crate::*,
    std::{
        rc::Rc,
        ptr::null_mut,
    },
};

pub struct ComputeShader {
    pub gpu: Rc<Gpu>,
    pub vk_shader_module: ffi::VkShaderModule,
}

impl ComputeShader {

}

impl Drop for ComputeShader {
    
    fn drop(&mut self) {
        unsafe { ffi::vkDestroyShaderModule(self.gpu.vk_device,self.vk_shader_module,null_mut()) };
    }
}
