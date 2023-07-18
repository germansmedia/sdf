use {
    crate::*,
    std::{
        rc::Rc,
        ptr::null_mut,
    },
};

pub struct Semaphore {
    pub gpu: Rc<Gpu>,
    pub vk_semaphore: ffi::VkSemaphore,
}

impl Drop for Semaphore {

    fn drop(&mut self) {
        unsafe { ffi::vkDestroySemaphore(self.gpu.vk_device,self.vk_semaphore,null_mut()) };
    }
}
