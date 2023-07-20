use {
    crate::*,
    std::{
        rc::Rc,
        ptr::null_mut,
        mem::MaybeUninit,
    },
};

impl PipelineLayout {

    pub fn create_descriptor_set(self: &Rc<Self>) -> Result<DescriptorSet,String> {

        let info = ffi::VkDescriptorSetAllocateInfo {
            sType: ffi::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
            pNext: null_mut(),
            descriptorPool: self.gpu.vk_descriptor_pool,
            descriptorSetCount: 1,
            pSetLayouts: &self.vk_descriptor_set_layout,
        };
        let mut vk_descriptor_set = MaybeUninit::uninit();
        match unsafe { ffi::vkAllocateDescriptorSets(self.gpu.vk_device,&info,vk_descriptor_set.as_mut_ptr()) } {
            ffi::VK_SUCCESS => Ok(DescriptorSet {
                gpu: Rc::clone(&self.gpu),
                vk_descriptor_set: unsafe { vk_descriptor_set.assume_init() },
            }),
            code => Err(format!("VulkanGpu::create_surface: Unable to create Vulkan XCB surface ({})",vk_code_to_string(code))),
        }
    }
}

pub enum DescriptorBinding {
    StorageImage,
    UniformBuffer,
}

pub enum Descriptor {
    StorageImage(*mut u8),
    UniformBuffer(*mut u8,u64),
}

pub struct DescriptorSet {
    pub gpu: Rc<Gpu>,
    pub vk_descriptor_set: ffi::VkDescriptorSet,
}

impl DescriptorSet {

    pub fn update(&self,index: usize,descriptor: Descriptor) {
        let descriptor_write = ffi::VkWriteDescriptorSet {
            sType: ffi::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            pNext: null_mut(),
            dstSet: self.vk_descriptor_set,
            dstBinding: index as u32,
            dstArrayElement: 0,
            descriptorCount: 1,
            descriptorType: match descriptor {
                Descriptor::StorageImage(_) => ffi::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
                Descriptor::UniformBuffer(_,_) => ffi::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            },
            pImageInfo: match descriptor {
                Descriptor::StorageImage(todo_ptr) => &ffi::VkDescriptorImageInfo {
                    sampler: null_mut(),
                    imageView: todo_ptr as ffi::VkImageView,
                    imageLayout: ffi::VK_IMAGE_LAYOUT_GENERAL,
                },
                _ => null_mut(),
            },
            pBufferInfo: match descriptor {
                Descriptor::UniformBuffer(todo_ptr,todo_size) => &ffi::VkDescriptorBufferInfo {
                    buffer: todo_ptr as ffi::VkBuffer,
                    offset: 0,
                    range: todo_size,
                },
                _ => null_mut(),
            },
            pTexelBufferView: null_mut(),
        };
        unsafe { ffi::vkUpdateDescriptorSets(self.gpu.vk_device,1,&descriptor_write,0,null_mut()) };
    }
}

impl Drop for DescriptorSet {

    fn drop(&mut self) {
        unsafe { ffi::vkFreeDescriptorSets(self.gpu.vk_device,self.gpu.vk_descriptor_pool,1,&self.vk_descriptor_set) };
    }
}
