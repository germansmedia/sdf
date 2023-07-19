use {
    crate::*,
    std::{
        //result::Result,
        rc::Rc,
        ptr::null_mut,
    },
};

pub struct Fence {
    pub gpu: Rc<Gpu>,
    pub vk_fence: ffi::VkFence,
}

impl Fence {

    pub fn wait(&self) -> Result<(),String> {
        unsafe { ffi::vkWaitForFences(self.gpu.vk_device,1,&self.vk_fence,ffi::VK_TRUE,0xFFFFFFFFFFFFFFFF) };
        Ok(())
    }

    pub fn reset(&self) -> Result<(),String> {
        unsafe { ffi::vkResetFences(self.gpu.vk_device,1,&self.vk_fence) };
        Ok(())
    }
}

impl Drop for Fence {

    fn drop(&mut self) {
        unsafe { ffi::vkDestroyFence(self.gpu.vk_device,self.vk_fence,null_mut()) };
    }
}
