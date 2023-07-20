use {
    crate::*,
    std::{
        //result::Result,
        rc::Rc,
        ptr::null_mut,
        mem::MaybeUninit,
    },
};

impl Gpu {

    pub fn create_fence(self: &Rc<Self>) -> Result<Fence,String> {

        let info = ffi::VkFenceCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
        };
        let mut vk_fence = MaybeUninit::uninit();
        match unsafe { ffi::vkCreateFence(self.vk_device,&info,null_mut(),vk_fence.as_mut_ptr()) } {
            ffi::VK_SUCCESS => Ok(Fence {
                gpu: Rc::clone(&self),
                vk_fence: unsafe { vk_fence.assume_init() },
            }),
            code => Err(format!("Gpu::create_fence: unable to create fence ({})",vk_code_to_string(code))),
        }
    }
}

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
