use {
    crate::*,
    std::{
        result::Result,
        rc::Rc,
        mem::MaybeUninit,
        ptr::null_mut,
    },
};

pub struct PipelineLayout {
    pub gpu: Rc<Gpu>,
    pub vk_pipeline_layout: ffi::VkPipelineLayout,
    pub vk_descriptor_set_layout: ffi::VkDescriptorSetLayout,
}

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
                vk_descriptor_set: { let vk = unsafe { vk_descriptor_set.assume_init() }; println!("CREATED descriptor set {:?}",vk); vk },
            }),
            code => Err(format!("VulkanGpu::create_surface: Unable to create Vulkan XCB surface ({})",vk_code_to_string(code))),
        }
    }
}

impl Drop for PipelineLayout {

    fn drop(&mut self) {
        unsafe {
            ffi::vkDestroyPipelineLayout(self.gpu.vk_device,self.vk_pipeline_layout,null_mut());
            ffi::vkDestroyDescriptorSetLayout(self.gpu.vk_device,self.vk_descriptor_set_layout,null_mut());
        };
    }
}
