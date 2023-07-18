use {
    crate::*,
    std::{
        result::Result,
        rc::Rc,
        ptr::null_mut,
    },
};

pub struct Surface {
    pub gpu: Rc<Gpu>,
    pub window: Rc<Window>,
    pub vk_surface: ffi::VkSurfaceKHR,
    pub vk_swapchain: ffi::VkSwapchainKHR,
    pub vk_image_views: Vec<ffi::VkImageView>,
}

impl Surface {

    pub fn set_rect(&mut self,r: Rect<i32>) -> Result<(),String> {

        if self.vk_image_views.len() > 0 {
            self.vk_image_views.iter().for_each(|vk_image_view| unsafe { ffi::vkDestroyImageView(self.gpu.vk_device,*vk_image_view,null_mut()) });
            unsafe { ffi::vkDestroySwapchainKHR(self.gpu.vk_device,self.vk_swapchain,null_mut()); }    
        }
        let (vk_swapchain,vk_image_views) = self.gpu.build_swapchain_resources(self.vk_surface,r)?;
        self.vk_swapchain = vk_swapchain;
        self.vk_image_views = vk_image_views;

        Ok(())
    }

    pub fn get_swapchain_count(&self) -> usize {
        self.vk_image_views.len()
    }

    pub fn acquire(&self) -> Result<usize,String> {
        let mut index = 0u32;
        match unsafe {
            ffi::vkAcquireNextImageKHR(
                self.gpu.vk_device,
                self.vk_swapchain,
                0xFFFFFFFFFFFFFFFF,
                // TODO: these cannot both be null:
                null_mut(),
                null_mut(),
                &mut index,
            )
        } {
            ffi::VK_SUCCESS => Ok(index as usize),
            code => Err(format!("VulkanSurface::acquire: unable to acquire next image ({})",super::vk_code_to_string(code))),
        }
    }

    pub fn present(&self,index: usize,wait_semaphore: Option<&Semaphore>) -> Result<(),String> {
        let image_index = index as u32;
        let info = ffi::VkPresentInfoKHR {
            sType: ffi::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
            pNext: null_mut(),
            waitSemaphoreCount: if let Some(_) = wait_semaphore { 1 } else { 0 },
            pWaitSemaphores: if let Some(semaphore) = wait_semaphore { &semaphore.vk_semaphore } else { null_mut() },
            swapchainCount: 1,
            pSwapchains: &self.vk_swapchain,
            pImageIndices: &image_index,
            pResults: null_mut(),
        };
        match unsafe { ffi::vkQueuePresentKHR(self.gpu.vk_queue,&info) } {
            ffi::VK_SUCCESS => Ok(()),
            code => Err(format!("VulkanSurface::present: unable to present image ({})",super::vk_code_to_string(code))),
        }
    }
}

impl Drop for Surface {

    fn drop(&mut self) {
        unsafe {
            ffi::vkDestroySwapchainKHR(self.gpu.vk_device,self.vk_swapchain,null_mut());
            ffi::vkDestroySurfaceKHR(self.gpu.vk_instance,self.vk_surface,null_mut());
        }
    }
}
