use {
    crate::*,
    std::{
        result::Result,
        rc::Rc,
        mem::MaybeUninit,
        ptr::null_mut,
    },
};

impl Gpu {

    pub fn create_pipeline_layout(self: &Rc<Self>,descriptor_bindings: &[DescriptorBinding]) -> Result<PipelineLayout,String> {

        let mut bindings: Vec<ffi::VkDescriptorSetLayoutBinding> = Vec::new();
        for i in 0usize..descriptor_bindings.len() {
            bindings.push(ffi::VkDescriptorSetLayoutBinding {
                binding: i as u32,
                descriptorType: match descriptor_bindings[i] {
                    DescriptorBinding::StorageImage => ffi::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
                    DescriptorBinding::UniformBuffer => ffi::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
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
}

pub struct PipelineLayout {
    pub gpu: Rc<Gpu>,
    pub vk_pipeline_layout: ffi::VkPipelineLayout,
    pub vk_descriptor_set_layout: ffi::VkDescriptorSetLayout,
}

impl Drop for PipelineLayout {

    fn drop(&mut self) {
        unsafe {
            ffi::vkDestroyPipelineLayout(self.gpu.vk_device,self.vk_pipeline_layout,null_mut());
            ffi::vkDestroyDescriptorSetLayout(self.gpu.vk_device,self.vk_descriptor_set_layout,null_mut());
        };
    }
}
