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

impl System {

    pub fn create_gpu(self: &Rc<System>) -> Result<Rc<Gpu>,String> {

        // create instance
        let extension_names = [
            ffi::VK_KHR_SURFACE_EXTENSION_NAME.as_ptr(),
#[cfg(system="linux")]            
            ffi::VK_KHR_XCB_SURFACE_EXTENSION_NAME.as_ptr(),
        ];
        let info = ffi::VkInstanceCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            pApplicationInfo: &ffi::VkApplicationInfo {
                sType: ffi::VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: null_mut(),
                pApplicationName: b"System\0".as_ptr() as *const i8,
                applicationVersion: (1 << 22) as u32,
                pEngineName: b"VulkanGpu\0".as_ptr() as *const i8,
                engineVersion: (1 << 22) as u32,
                apiVersion: ((1 << 22) | (2 << 11)) as u32,
            },
            enabledExtensionCount: extension_names.len() as u32,
            ppEnabledExtensionNames: extension_names.as_ptr() as *const *const i8,
            enabledLayerCount: 0,
            ppEnabledLayerNames: null_mut(),
        };
        let mut vk_instance = MaybeUninit::<ffi::VkInstance>::uninit();
        match unsafe { ffi::vkCreateInstance(&info,null_mut(),vk_instance.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => return Err(format!("System::create_gpu: unable to create VkInstance ({})",vk_code_to_string(code))),
        }
        let vk_instance = unsafe { vk_instance.assume_init() };

        // enumerate physical devices
        let mut count = MaybeUninit::<u32>::uninit();
        unsafe { ffi::vkEnumeratePhysicalDevices(vk_instance,count.as_mut_ptr(),null_mut()) };
        let count = unsafe { count.assume_init() };
        if count == 0 {
            unsafe { ffi::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("System::create_gpu: unable to enumerate physical devices".to_string());
        }
        let mut vk_physical_devices = vec![null_mut(); count as usize];
        unsafe { ffi::vkEnumeratePhysicalDevices(vk_instance,&count as *const u32 as *mut u32,vk_physical_devices.as_mut_ptr()) };

        println!("System::create_gpu: physical devices:");
        vk_physical_devices.iter().for_each(|vk_physical_device| {
            let mut properties = MaybeUninit::<ffi::VkPhysicalDeviceProperties>::uninit();
            unsafe { ffi::vkGetPhysicalDeviceProperties(*vk_physical_device,properties.as_mut_ptr()) };
            let properties = unsafe { properties.assume_init() };
            let slice: &[u8] = unsafe { &*(&properties.deviceName as *const [i8] as *const [u8]) };
            println!("System::create_gpu:     {}",std::str::from_utf8(slice).unwrap());
        });

        // get first physical device
        println!("System::create_gpu: choosing first device");
        let vk_physical_device = vk_physical_devices[0];
        
        // get supported queue families
        let mut count = 0u32;
        unsafe { ffi::vkGetPhysicalDeviceQueueFamilyProperties(vk_physical_device,&mut count as *mut u32,null_mut()) };
        if count == 0 {
            unsafe { ffi::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("System::create_gpu: no queue families supported on this GPU".to_string());
        }
        let mut vk_queue_families = vec![MaybeUninit::<ffi::VkQueueFamilyProperties>::uninit(); count as usize];
        unsafe { ffi::vkGetPhysicalDeviceQueueFamilyProperties(
            vk_physical_device,
            &count as *const u32 as *mut u32,
            vk_queue_families.as_mut_ptr() as *mut ffi::VkQueueFamilyProperties,
        ) };
        let vk_queue_families = unsafe { std::mem::transmute::<_,Vec<ffi::VkQueueFamilyProperties>>(vk_queue_families) };

        // DEBUG: display the number of queues and capabilities
        println!("System::create_gpu: supported queue families:");
        vk_queue_families.iter().for_each(|vk_queue_family| {
            let mut capabilities = String::new();
            if vk_queue_family.queueFlags & ffi::VK_QUEUE_GRAPHICS_BIT != 0 {
                capabilities.push_str("graphics ");
            }
            if vk_queue_family.queueFlags & ffi::VK_QUEUE_TRANSFER_BIT != 0 {
                capabilities.push_str("transfer ");
            }
            if vk_queue_family.queueFlags & ffi::VK_QUEUE_COMPUTE_BIT != 0 {
                capabilities.push_str("compute ");
            }
            if vk_queue_family.queueFlags & ffi::VK_QUEUE_SPARSE_BINDING_BIT != 0 {
                capabilities.push_str("sparse ");
            }
            println!("System::create_gpu:     - {} queues, capable of: {}",vk_queue_family.queueCount,capabilities);
        });

        // assume the first queue family is the one we want for all queues
        println!("System::create_gpu: choosing first family");
        let vk_queue_family = vk_queue_families[0];
        let mask = ffi::VK_QUEUE_GRAPHICS_BIT | ffi::VK_QUEUE_TRANSFER_BIT | ffi::VK_QUEUE_COMPUTE_BIT;
        if (vk_queue_family.queueFlags & mask) != mask {
            unsafe { ffi::vkDestroyInstance(vk_instance,null_mut()) };
            return Err("System::create_gpu: first queue family does not support graphics, transfer and compute operations".to_string());
        }

        // assume that presentation is done on the same family as graphics and create logical device with one queue of queue family 0
        let mut queue_create_infos = Vec::<ffi::VkDeviceQueueCreateInfo>::new();
        let priority = 1f32;
        queue_create_infos.push(ffi::VkDeviceQueueCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            queueFamilyIndex: 0,
            queueCount: 1,
            pQueuePriorities: &priority as *const f32,
        });
        let extension_names = [
            ffi::VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr(),
        ];
        let info = ffi::VkDeviceCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            queueCreateInfoCount: queue_create_infos.len() as u32,
            pQueueCreateInfos: queue_create_infos.as_ptr(),
            enabledLayerCount: 0,
            ppEnabledLayerNames: null_mut(),
            enabledExtensionCount: extension_names.len() as u32,
            ppEnabledExtensionNames: extension_names.as_ptr() as *const *const i8,
            pEnabledFeatures: &ffi::VkPhysicalDeviceFeatures {
                robustBufferAccess: 0,
                fullDrawIndexUint32: 0,
                imageCubeArray: 0,
                independentBlend: 0,
                geometryShader: 0,
                tessellationShader: 0,
                sampleRateShading: 0,
                dualSrcBlend: 0,
                logicOp: 0,
                multiDrawIndirect: 0,
                drawIndirectFirstInstance: 0,
                depthClamp: 0,
                depthBiasClamp: 0,
                fillModeNonSolid: 0,
                depthBounds: 0,
                wideLines: 0,
                largePoints: 0,
                alphaToOne: 0,
                multiViewport: 0,
                samplerAnisotropy: 0,
                textureCompressionETC2: 0,
                textureCompressionASTC_LDR: 0,
                textureCompressionBC: 0,
                occlusionQueryPrecise: 0,
                pipelineStatisticsQuery: 0,
                vertexPipelineStoresAndAtomics: 0,
                fragmentStoresAndAtomics: 0,
                shaderTessellationAndGeometryPointSize: 0,
                shaderImageGatherExtended: 0,
                shaderStorageImageExtendedFormats: 0,
                shaderStorageImageMultisample: 0,
                shaderStorageImageReadWithoutFormat: 0,
                shaderStorageImageWriteWithoutFormat: 1,
                shaderUniformBufferArrayDynamicIndexing: 0,
                shaderSampledImageArrayDynamicIndexing: 0,
                shaderStorageBufferArrayDynamicIndexing: 0,
                shaderStorageImageArrayDynamicIndexing: 0,
                shaderClipDistance: 0,
                shaderCullDistance: 0,
                shaderFloat64: 0,
                shaderInt64: 0,
                shaderInt16: 0,
                shaderResourceResidency: 0,
                shaderResourceMinLod: 0,
                sparseBinding: 0,
                sparseResidencyBuffer: 0,
                sparseResidencyImage2D: 0,
                sparseResidencyImage3D: 0,
                sparseResidency2Samples: 0,
                sparseResidency4Samples: 0,
                sparseResidency8Samples: 0,
                sparseResidency16Samples: 0,
                sparseResidencyAliased: 0,
                variableMultisampleRate: 0,
                inheritedQueries: 0,
            },
        };
        let mut vk_device = MaybeUninit::<ffi::VkDevice>::uninit();
        match unsafe { ffi::vkCreateDevice(vk_physical_device,&info,null_mut(),vk_device.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => { 
                unsafe { ffi::vkDestroyInstance(vk_instance,null_mut()) };
                return Err(format!("System::create_gpu: unable to create VkDevice ({})",vk_code_to_string(code)));
            },
        }
        let vk_device = unsafe { vk_device.assume_init() };

        // obtain the queue from queue family 0
        let mut vk_queue = MaybeUninit::<ffi::VkQueue>::uninit();
        unsafe { ffi::vkGetDeviceQueue(vk_device,0,0,vk_queue.as_mut_ptr()) };
        let vk_queue = unsafe { vk_queue.assume_init() };

        // create command pool for this queue
        let info = ffi::VkCommandPoolCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: null_mut(),
            flags: ffi::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
            queueFamilyIndex: 0,
        };
        let mut vk_command_pool = MaybeUninit::<ffi::VkCommandPool>::uninit();
        match unsafe { ffi::vkCreateCommandPool(vk_device,&info,null_mut(),vk_command_pool.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => {
                unsafe {
                    ffi::vkDestroyDevice(vk_device,null_mut());
                    ffi::vkDestroyInstance(vk_instance,null_mut());
                }
                return Err(format!("System::create_gpu: unable to create command pool ({})",vk_code_to_string(code)));
            },
        }
        let vk_command_pool = unsafe { vk_command_pool.assume_init() };

        // create descriptor pool
        let info = ffi::VkDescriptorPoolCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
            pNext: null_mut(),
            flags: ffi::VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT,
            maxSets: 5u32,
            poolSizeCount: 1,
            pPoolSizes: &ffi::VkDescriptorPoolSize {
                type_: ffi::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
                descriptorCount: 5u32,
            },
        };
        let mut vk_descriptor_pool = MaybeUninit::uninit();
        match unsafe { ffi::vkCreateDescriptorPool(vk_device,&info,null_mut(),vk_descriptor_pool.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => {
                unsafe {
                    ffi::vkDestroyDevice(vk_device,null_mut());
                    ffi::vkDestroyInstance(vk_instance,null_mut());
                }
                return Err(format!("System::create_gpu: unable to create descriptor pool ({})",vk_code_to_string(code)));
            }
        }
        let vk_descriptor_pool = unsafe { vk_descriptor_pool.assume_init() };

        // get memory properties
        let mut vk_memory_properties = MaybeUninit::<ffi::VkPhysicalDeviceMemoryProperties>::uninit();
        unsafe { ffi::vkGetPhysicalDeviceMemoryProperties(vk_physical_device,vk_memory_properties.as_mut_ptr()) };
        let vk_memory_properties = unsafe { vk_memory_properties.assume_init() };

        // DEBUG: show the entire memory description
        println!("System::create_gpu: device memory properties:");
        println!("System::create_gpu:     memory heaps:");
        for i in 0..vk_memory_properties.memoryHeapCount as usize {
            println!("System::create_gpu:         {}: size {} MiB, {:X}",i,vk_memory_properties.memoryHeaps[i].size / (1024 * 1024),vk_memory_properties.memoryHeaps[i].flags);
        }
        println!("System::create_gpu:     memory types:");
        for i in 0..vk_memory_properties.memoryTypeCount as usize {
            let mut flags = String::new();
            let vk_memory_type = &vk_memory_properties.memoryTypes[i];
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT) != 0 {
                flags += "device_local ";
            }
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT) != 0 {
                flags += "host_visible ";
            }
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT) != 0 {
                flags += "host_coherent ";
            }
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_HOST_CACHED_BIT) != 0 {
                flags += "host_cached ";
            }
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT) != 0 {
                flags += "lazily_allocated ";
            }
            if (vk_memory_type.propertyFlags & ffi::VK_MEMORY_PROPERTY_PROTECTED_BIT) != 0 {
                flags += "protected ";
            }
            println!("System::create_gpu:         - on heap {}, {}",vk_memory_type.heapIndex,flags);
        }

        // find shared memory heap and type (later also find device-only index)
        let mask = ffi::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT | ffi::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | ffi::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;
        let valid_types: Vec<(usize,&ffi::VkMemoryType)> = vk_memory_properties.memoryTypes.iter().enumerate().filter(|vk_memory_type| (vk_memory_type.1.propertyFlags & mask) == mask).collect();
        if valid_types.is_empty() {
            return Err("System::create_gpu: no valid memory types found".to_string());
        }
        let shared_index = valid_types[0].0;

        Ok(Rc::new(Gpu {
            system: Rc::clone(&self),
            vk_instance,
            vk_physical_device,
            vk_device,
            vk_queue,
            vk_command_pool,
            vk_descriptor_pool,
            shared_index,
        }))
    }
}

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

        Ok((vk_swapchain,vk_image_views))
    }
}
