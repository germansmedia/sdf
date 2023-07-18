use {
    crate::*,
    std::{
        rc::Rc,
        ptr::null_mut,
    },
};

pub enum DescriptorBinding {
    StorageImage,
}

pub enum Descriptor {
    StorageImage(*mut u8),
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
            },
            pImageInfo: match descriptor {
                Descriptor::StorageImage(todo_ptr) => &ffi::VkDescriptorImageInfo {
                    sampler: null_mut(),
                    imageView: todo_ptr as ffi::VkImageView,
                    imageLayout: ffi::VK_IMAGE_LAYOUT_GENERAL,
                },
            },
            pBufferInfo: null_mut(),
            pTexelBufferView: null_mut(),
        };
        unsafe { ffi::vkUpdateDescriptorSets(self.gpu.vk_device,1,&descriptor_write,0,null_mut()) };
    }
}

impl Drop for DescriptorSet {

    fn drop(&mut self) {
        // TODO
    }
}
