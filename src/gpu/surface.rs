use {
    crate::*,
    std::{
        result::Result,
        rc::Rc,
        ptr::null_mut,
        mem::MaybeUninit,
    },
};

pub struct Surface {
    pub gpu: Rc<Gpu>,
    pub window: Rc<Window>,
    pub vk_surface: ffi::VkSurfaceKHR,
    pub vk_swapchain: ffi::VkSwapchainKHR,
    pub vk_image_views: Vec<ffi::VkImageView>,
}

impl Gpu {

    pub fn create_surface(self: &Rc<Self>,window: Rc<Window>,r: Rect<i32>) -> Result<Surface,String> {

        // create surface for this window
        let vk_surface = {
            let info = ffi::VkXcbSurfaceCreateInfoKHR {
                sType: ffi::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
                pNext: null_mut(),
                flags: 0,
                connection: window.system.xcb_connection,
                window: window.xcb_window,
            };
            let mut vk_surface = MaybeUninit::<ffi::VkSurfaceKHR>::uninit();
            match unsafe { ffi::vkCreateXcbSurfaceKHR(self.vk_instance,&info,null_mut(),vk_surface.as_mut_ptr()) } {
                ffi::VK_SUCCESS => { },
                code => {
                    return Err(format!("Gpu::create_surface: Unable to create Vulkan XCB surface ({})",vk_code_to_string(code)));
                },
            }
            unsafe { vk_surface.assume_init() }
        };

        // verify the surface is supported for the current physical device
        let mut supported = MaybeUninit::<ffi::VkBool32>::uninit();
        match unsafe { ffi::vkGetPhysicalDeviceSurfaceSupportKHR(self.vk_physical_device,0,vk_surface,supported.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => {
                return Err(format!("Gpu::create_surface: Surface not supported on physical device ({})",vk_code_to_string(code)));
            },
        }
        let supported = unsafe { supported.assume_init() };
        if supported == ffi::VK_FALSE {
            return Err("Gpu::create_surface: Surface not supported on physical device".to_string());
        }

        let (vk_swapchain,vk_image_views) = self.build_swapchain_resources(vk_surface,&r)?;

        Ok(Surface {
            gpu: Rc::clone(&self),
            window,
            vk_surface,
            vk_swapchain,
            vk_image_views,
        })
    }
}

impl Surface {

    pub fn set_rect(&mut self,r: &Rect<i32>) -> Result<(),String> {

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

    pub fn acquire(&self,ready_semaphore: Option<&Semaphore>,ready_fence: Option<&Fence>) -> Result<usize,String> {
        let mut index = 0u32;
        match unsafe {
            ffi::vkAcquireNextImageKHR(
                self.gpu.vk_device,
                self.vk_swapchain,
                0xFFFFFFFFFFFFFFFF,
                if let Some(semaphore) = ready_semaphore { semaphore.vk_semaphore } else { null_mut() },
                if let Some(fence) = ready_fence { fence.vk_fence } else { null_mut() },
                &mut index,
            )
        } {
            ffi::VK_SUCCESS => Ok(index as usize),
            code => Err(format!("Surface::acquire: unable to acquire next image ({})",vk_code_to_string(code))),
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
            code => Err(format!("Surface::present: unable to present image ({})",vk_code_to_string(code))),
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
