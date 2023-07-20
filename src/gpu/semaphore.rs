use {
    crate::*,
    std::{
        rc::Rc,
        ptr::null_mut,
        mem::MaybeUninit,
    },
};

impl Gpu {

    pub fn create_semaphore(self: &Rc<Self>) -> Result<Semaphore,String> {

        /*
        let type_info = ffi::VkSemaphoreTypeCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO,
            pNext: null_mut(),
            semaphoreType: ffi::VK_SEMAPHORE_TYPE_TIMELINE,
            initialValue: 0,
        };
        */
        let info = ffi::VkSemaphoreCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            //pNext: &type_info as *const ffi::VkSemaphoreTypeCreateInfo as *const c_void,
            pNext: null_mut(),
            flags: 0,
        };
        let mut vk_semaphore = MaybeUninit::uninit();
        match unsafe { ffi::vkCreateSemaphore(self.vk_device,&info,null_mut(),vk_semaphore.as_mut_ptr()) } {
            ffi::VK_SUCCESS => Ok(Semaphore {
                gpu: Rc::clone(&self),
                vk_semaphore: unsafe { vk_semaphore.assume_init() },
            }),
            code => Err(format!("Gpu::create_semaphore: unable to create semaphore ({})",vk_code_to_string(code))),
        }
    }
}

pub struct Semaphore {
    pub gpu: Rc<Gpu>,
    pub vk_semaphore: ffi::VkSemaphore,
}

impl Drop for Semaphore {

    fn drop(&mut self) {
        unsafe { ffi::vkDestroySemaphore(self.gpu.vk_device,self.vk_semaphore,null_mut()) };
    }
}
