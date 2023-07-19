use {
    crate::*,
    std::{
        result::Result,
        rc::Rc,
        mem::MaybeUninit,
        ptr::null_mut,
        //ffi::c_void,
    },
};

pub struct Gpu {
    pub system: Rc<System>,
    pub vk_instance: ffi::VkInstance,
    pub vk_physical_device: ffi::VkPhysicalDevice,
    pub vk_device: ffi::VkDevice,
    pub vk_queue: ffi::VkQueue,
    pub vk_command_pool: ffi::VkCommandPool,
    pub vk_descriptor_pool: ffi::VkDescriptorPool,
    pub shared_index: usize,
}

impl Gpu {

    pub fn build_swapchain_resources(&self,vk_surface: ffi::VkSurfaceKHR,r: &Rect<i32>) -> Result<(ffi::VkSwapchainKHR,Vec<ffi::VkImageView>),String> {

        // get surface capabilities to calculate the extent and image count
        let mut capabilities = MaybeUninit::<ffi::VkSurfaceCapabilitiesKHR>::uninit();
        match unsafe { ffi::vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
            self.vk_physical_device,
            vk_surface,
            capabilities.as_mut_ptr(),
        ) } {
            ffi::VK_SUCCESS => { },
            code => {
                return Err(format!("Gpu::build_swapchain_resources: unable to get surface capabilities ({})",vk_code_to_string(code)));
            },
        }
        let capabilities = unsafe { capabilities.assume_init() };

        // get current extent, if any
        let extent = if capabilities.currentExtent.width != 0xFFFFFFFF {
            Vec2 {
                x: capabilities.currentExtent.width,
                y: capabilities.currentExtent.height,
            }
        }

        // otherwise take window size as extent, and make sure it fits the constraints
        else {
            let mut extent = Vec2 { x: r.s.x as u32,y: r.s.y as u32, };
            if extent.x < capabilities.minImageExtent.width {
                extent.x = capabilities.minImageExtent.width;
            }
            if extent.y < capabilities.minImageExtent.height {
                extent.y = capabilities.minImageExtent.height;
            }
            if extent.x > capabilities.maxImageExtent.width {
                extent.x = capabilities.maxImageExtent.width;
            }
            if extent.y > capabilities.maxImageExtent.height {
                extent.y = capabilities.maxImageExtent.height;
            }
            extent
        };

        // make sure VK_FORMAT_B8G8R8A8_SRGB is supported (BGRA8UN)
        let mut count = 0u32;
        match unsafe { ffi::vkGetPhysicalDeviceSurfaceFormatsKHR(
            self.vk_physical_device,
            vk_surface,
            &mut count as *mut u32,
            null_mut(),
        ) } {
            ffi::VK_SUCCESS => { },
            code => {
                return Err(format!("Gpu::build_swapchain_resources: unable to get surface formats ({})",vk_code_to_string(code)));
            },
        }
        let mut formats = vec![MaybeUninit::<ffi::VkSurfaceFormatKHR>::uninit(); count as usize];
        match unsafe { ffi::vkGetPhysicalDeviceSurfaceFormatsKHR(
            self.vk_physical_device,
            vk_surface,
            &mut count,
            formats.as_mut_ptr() as *mut ffi::VkSurfaceFormatKHR,
        ) } {
            ffi::VK_SUCCESS => { },
            code => {
                return Err(format!("Gpu::build_swapchain_resources: unable to get surface formats ({})",vk_code_to_string(code)));
            }
        }
        let formats = unsafe { std::mem::transmute::<_,Vec<ffi::VkSurfaceFormatKHR>>(formats) };
        let mut format_supported = false;
        println!("Gpu::build_swapchain_resources: supported formats:");
        for format in formats.iter() {
            println!("Gpu::build_swapchain_resources:     {} at space {}",vk_format_to_string(format.format),vk_colorspace_to_string(format.colorSpace));
            if (format.format == ffi::VK_FORMAT_B8G8R8A8_UNORM) && (format.colorSpace == ffi::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR) {
                format_supported = true;
            }
        }
        if !format_supported {
            return Err("Gpu::build_swapchain_resources: window does not support BGRA8UN".to_string());
        }

        // create swapchain for this window
        let info = ffi::VkSwapchainCreateInfoKHR {
            sType: ffi::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            pNext: null_mut(),
            flags: 0,
            surface: vk_surface,
            minImageCount: capabilities.minImageCount,
            imageFormat: ffi::VK_FORMAT_B8G8R8A8_UNORM,
            imageColorSpace: ffi::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
            imageExtent: ffi::VkExtent2D { width: extent.x,height: extent.y, },
            imageArrayLayers: 1,
            imageUsage: ffi::VK_IMAGE_USAGE_TRANSFER_DST_BIT | ffi::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | ffi::VK_IMAGE_USAGE_STORAGE_BIT,
            imageSharingMode: ffi::VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: null_mut(),
            preTransform: capabilities.currentTransform,
            compositeAlpha: ffi::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            presentMode: ffi::VK_PRESENT_MODE_FIFO_KHR,
            clipped: ffi::VK_TRUE,
            oldSwapchain: null_mut(),
        };    
        println!("creating swapchain");
        let mut vk_swapchain: ffi::VkSwapchainKHR = null_mut();
        match unsafe { ffi::vkCreateSwapchainKHR(
            self.vk_device,
            &info,
            null_mut(),
            &mut vk_swapchain as *mut ffi::VkSwapchainKHR,
        ) } {
            ffi::VK_SUCCESS => { },
            code => {
                return Err(format!("Gpu::build_swapchain_resources: unable to create swap chain ({})",vk_code_to_string(code)));
            },
        }

        // get swapchain images
        println!("getting swapchain images");
        let mut count = 0u32;
        match unsafe { ffi::vkGetSwapchainImagesKHR(self.vk_device,vk_swapchain,&mut count as *mut u32,null_mut()) } {
            ffi::VK_SUCCESS => { },
            code => {
                unsafe { ffi::vkDestroySwapchainKHR(self.vk_device,vk_swapchain,null_mut()) };
                return Err(format!("Gpu::build_swapchain_resources: unable to get swap chain image count ({})",vk_code_to_string(code)));
            },
        }
        let mut vk_images = vec![MaybeUninit::<ffi::VkImage>::uninit(); count as usize];
        match unsafe { ffi::vkGetSwapchainImagesKHR(
            self.vk_device,
            vk_swapchain,
            &count as *const u32 as *mut u32,
            vk_images.as_mut_ptr() as *mut ffi::VkImage,
        ) } {
            ffi::VK_SUCCESS => { },
            code => {
                unsafe { ffi::vkDestroySwapchainKHR(self.vk_device,vk_swapchain,null_mut()) };
                return Err(format!("Gpu::build_swapchain_resources: unable to get swap chain images ({})",vk_code_to_string(code)));
            },
        }
        let vk_images = unsafe { std::mem::transmute::<_,Vec<ffi::VkImage>>(vk_images) };

        // create image views for the swapchain images
        println!("creating swapchain image views");
        let results: Vec<Result<ffi::VkImageView,String>> = vk_images.iter().map(|vk_image| {
            let info = ffi::VkImageViewCreateInfo {
                sType: ffi::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                pNext: null_mut(),
                flags: 0,
                image: *vk_image,
                viewType: ffi::VK_IMAGE_VIEW_TYPE_2D,
                format: ffi::VK_FORMAT_B8G8R8A8_UNORM,
                components: ffi::VkComponentMapping {
                    r: ffi::VK_COMPONENT_SWIZZLE_IDENTITY,
                    g: ffi::VK_COMPONENT_SWIZZLE_IDENTITY,
                    b: ffi::VK_COMPONENT_SWIZZLE_IDENTITY,
                    a: ffi::VK_COMPONENT_SWIZZLE_IDENTITY,
                },
                subresourceRange: ffi::VkImageSubresourceRange {
                    aspectMask: ffi::VK_IMAGE_ASPECT_COLOR_BIT,
                    baseMipLevel: 0,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                },
            };
            let mut vk_image_view: ffi::VkImageView = null_mut();
            match unsafe { ffi::vkCreateImageView(self.vk_device,&info,null_mut(),&mut vk_image_view) } {
                ffi::VK_SUCCESS => Ok(vk_image_view),
                code => Err(format!("Gpu::build_swapchain_resources: unable to create image view ({})",vk_code_to_string(code))),
            }
        }).collect();
        if results.iter().any(|result| result.is_err()) {
            results.iter().for_each(|result| if let Ok(vk_image_view) = result { unsafe { ffi::vkDestroyImageView(self.vk_device,*vk_image_view,null_mut()) } });
            unsafe { ffi::vkDestroySwapchainKHR(self.vk_device,vk_swapchain,null_mut()); }
            return Err("Gpu::build_swapchain_resources: unable to create image view".to_string());
        }
        let vk_image_views: Vec<ffi::VkImageView> = results.into_iter().map(|result| result.unwrap()).collect();

        println!("rebuilt swapchain");

        Ok((vk_swapchain,vk_image_views))
    }

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

    pub fn create_command_buffer(self: &Rc<Self>) -> Result<CommandBuffer,String> {

        let info = ffi::VkCommandBufferAllocateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: null_mut(),
            commandPool: self.vk_command_pool,
            level: ffi::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            commandBufferCount: 1,
        };
        let mut vk_command_buffer = MaybeUninit::uninit();
        match unsafe { ffi::vkAllocateCommandBuffers(self.vk_device,&info,vk_command_buffer.as_mut_ptr()) } {
            ffi::VK_SUCCESS => Ok(CommandBuffer {
                gpu: Rc::clone(&self),
                vk_command_buffer: unsafe { vk_command_buffer.assume_init() },
                compute_pipeline: None,
                pipeline_layout: None,
                descriptor_set: None,
            }),
            code => Err(format!("Gpu::create_command_buffer: unable to create command buffer ({})",vk_code_to_string(code))),
        }
    }

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

    pub fn create_pipeline_layout(self: &Rc<Self>,descriptor_bindings: &[DescriptorBinding]) -> Result<PipelineLayout,String> {

        let mut bindings: Vec<ffi::VkDescriptorSetLayoutBinding> = Vec::new();
        for i in 0usize..descriptor_bindings.len() {
            bindings.push(ffi::VkDescriptorSetLayoutBinding {
                binding: i as u32,
                descriptorType: match descriptor_bindings[i] {
                    DescriptorBinding::StorageImage => ffi::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
                },
                descriptorCount: 1,
                stageFlags: ffi::VK_SHADER_STAGE_COMPUTE_BIT,
                pImmutableSamplers: null_mut(),
            });
        }
        let info = ffi::VkDescriptorSetLayoutCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            bindingCount: bindings.len() as u32,
            pBindings: bindings.as_ptr(),
        };
        let mut vk_descriptor_set_layout = MaybeUninit::uninit();
        let vk_descriptor_set_layout = match unsafe { ffi::vkCreateDescriptorSetLayout(self.vk_device,&info,null_mut(),vk_descriptor_set_layout.as_mut_ptr()) } {
            ffi::VK_SUCCESS => unsafe { vk_descriptor_set_layout.assume_init() },
            code => return Err(format!("Gpu::create_pipeline_layout: unable to create descriptor set layout ({})",vk_code_to_string(code))),
        };
        let info = ffi::VkPipelineLayoutCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            setLayoutCount: 1,
            pSetLayouts: &vk_descriptor_set_layout,
            pushConstantRangeCount: 0,
            pPushConstantRanges: null_mut(),
        };
        let mut vk_pipeline_layout = MaybeUninit::uninit();
        match unsafe { ffi::vkCreatePipelineLayout(self.vk_device,&info,null_mut(),vk_pipeline_layout.as_mut_ptr()) } {
            ffi::VK_SUCCESS => Ok(PipelineLayout {
                gpu: Rc::clone(&self),
                vk_pipeline_layout: unsafe { vk_pipeline_layout.assume_init() },
                vk_descriptor_set_layout,
            }),
            code => Err(format!("Gpu::create_pipeline_layout: nable to create pipeline layout ({})",vk_code_to_string(code))),
        }
    }

    pub fn submit_command_buffer(&self,command_buffer: &CommandBuffer,wait_semaphore: Option<&Semaphore>,signal_semaphore: Option<&Semaphore>,signal_fence: Option<&Fence>) -> Result<(),String> {

        let wait_stage = ffi::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
        let info = ffi::VkSubmitInfo {
            sType: ffi::VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: null_mut(),
            waitSemaphoreCount: if let Some(_) = wait_semaphore { 1 } else { 0 },
            pWaitSemaphores: if let Some(semaphore) = wait_semaphore { &semaphore.vk_semaphore } else { null_mut() },
            pWaitDstStageMask: &wait_stage,
            commandBufferCount: 1,
            pCommandBuffers: &command_buffer.vk_command_buffer,
            signalSemaphoreCount: if let Some(_) = signal_semaphore { 1 } else { 0 },
            pSignalSemaphores: if let Some(semaphore) = signal_semaphore { &semaphore.vk_semaphore } else { null_mut() },
        };
        match unsafe { ffi::vkQueueSubmit(self.vk_queue,1,&info,if let Some(fence) = signal_fence { fence.vk_fence } else { null_mut() }) } {
            ffi::VK_SUCCESS => Ok(()),
            code => Err(format!("Gpu::submit_command_buffer: unable to submit command buffer to graphics queue ({})",vk_code_to_string(code))),
        }
    }

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
