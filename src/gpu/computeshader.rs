use {
    crate::*,
    std::{
        rc::Rc,
        ptr::null_mut,
        mem::MaybeUninit,
    },
};

impl Gpu {

    pub fn create_compute_shader(self: &Rc<Self>,code: &[u8]) -> Result<ComputeShader,String> {

        let info = ffi::VkShaderModuleCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            codeSize: code.len() as u64,
            pCode: code.as_ptr() as *const u32,
        };
        let mut vk_shader_module = MaybeUninit::uninit();
        match unsafe { ffi::vkCreateShaderModule(self.vk_device,&info,null_mut(),vk_shader_module.as_mut_ptr()) } {
            ffi::VK_SUCCESS => Ok(ComputeShader {
                gpu: Rc::clone(&self),
                vk_shader_module: unsafe { vk_shader_module.assume_init() },
            }),
            code => Err(format!("Gpu::create_compute_shader: unable to create compute shader ({})",vk_code_to_string(code))),
        }
    }
}

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
