use {
    crate::*,
    std::{
        result::Result,
        rc::Rc,
        ptr::{
            null_mut,
            copy_nonoverlapping,
        },
        mem::{
            MaybeUninit,
            size_of,
        },
    },
};

impl Gpu {

    pub fn create_uniform_buffer<T>(self: &Rc<Self>,data: &T) -> Result<UniformBuffer,String> {

        let info = ffi::VkBufferCreateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: null_mut(),
            flags: 0,
            size: size_of::<T>() as u64,
            usage: ffi::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT,
            sharingMode: ffi::VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: null_mut(),
        };
        let mut vk_buffer = MaybeUninit::uninit();
        match unsafe { ffi::vkCreateBuffer(self.vk_device,&info,null_mut(),vk_buffer.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => return Err(format!("Gpu::create_uniform_buffer: unable to create buffer ({})",vk_code_to_string(code))),
        }
        let vk_buffer = unsafe { vk_buffer.assume_init() };

        let info = ffi::VkMemoryAllocateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            pNext: null_mut(),
            allocationSize: size_of::<T>() as u64,
            memoryTypeIndex: self.shared_index as u32,
        };
        let mut vk_memory = MaybeUninit::uninit();
        match unsafe { ffi::vkAllocateMemory(self.vk_device,&info,null_mut(),vk_memory.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => return Err(format!("Gpu::create_uniform_buffer: unable to allocate memory ({})",vk_code_to_string(code))),
        }
        let vk_memory = unsafe { vk_memory.assume_init() };

        let mut data_ptr = MaybeUninit::uninit();
        match unsafe { ffi::vkMapMemory(self.vk_device,vk_memory,0,ffi::VK_WHOLE_SIZE as u64,0,data_ptr.as_mut_ptr()) } {
            ffi::VK_SUCCESS => { },
            code => return Err(format!("Gpu::create_uniform_buffer: unable to map memory ({})",vk_code_to_string(code))),
        }
        let data_ptr = unsafe { data_ptr.assume_init() } as *mut u8;

        unsafe { copy_nonoverlapping(data as *const T as *const u8,data_ptr,size_of::<T>()) };

        match unsafe { ffi::vkBindBufferMemory(self.vk_device,vk_buffer,vk_memory,0) } {
            ffi::VK_SUCCESS => Ok(UniformBuffer {
                gpu: Rc::clone(&self),
                vk_buffer,
                vk_memory,
                data_ptr,
                size: size_of::<T>(),
            }),
            code => return Err(format!("Gpu::create_uniform_buffer: unable to bind buffer memory ({})",vk_code_to_string(code))),
        }
    }
}

pub struct UniformBuffer {
    pub gpu: Rc<Gpu>,
    pub vk_buffer: ffi::VkBuffer,
    pub vk_memory: ffi::VkDeviceMemory,
    pub data_ptr: *mut u8,
    pub size: usize,
}

impl UniformBuffer {

    pub fn update<T>(&self,data: &T) {
        unsafe { copy_nonoverlapping(data as *const T as *const u8,self.data_ptr,size_of::<T>()) };
    }
}

impl Drop for UniformBuffer {
    fn drop(&mut self) {
        unsafe {
            ffi::vkDestroyBuffer(self.gpu.vk_device,self.vk_buffer,null_mut());
            ffi::vkFreeMemory(self.gpu.vk_device,self.vk_memory,null_mut());
        }
    }
}
