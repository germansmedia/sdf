use {
    crate::*,
    std::{
        result::Result,
        rc::Rc,
        ptr::null_mut,
        mem::MaybeUninit,
    },
};

impl Gpu {

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
}

pub struct CommandBuffer {
    pub gpu: Rc<Gpu>,
    pub vk_command_buffer: ffi::VkCommandBuffer,
    pub compute_pipeline: Option<Rc<ComputePipeline>>,
    pub pipeline_layout: Option<Rc<PipelineLayout>>,
    pub descriptor_set: Option<Rc<DescriptorSet>>,
}

impl CommandBuffer {

    pub fn reset(&mut self) -> Result<(),String> {

        self.pipeline_layout = None;
        self.descriptor_set = None;
        self.compute_pipeline = None;
        match unsafe { ffi::vkResetCommandBuffer(self.vk_command_buffer,0) } {
            ffi::VK_SUCCESS => Ok(()),
            code => Err(format!("CommandBuffer::reset: unable to reset command buffer ({})",vk_code_to_string(code))),
        }
    }

    pub fn begin(&self) -> Result<(),String> {

        let info = ffi::VkCommandBufferBeginInfo {
            sType: ffi::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: null_mut(),
            flags: 0,
            pInheritanceInfo: null_mut(),
        };
        match unsafe { ffi::vkBeginCommandBuffer(self.vk_command_buffer,&info) } {
            ffi::VK_SUCCESS => Ok(()),
            code => Err(format!("VulkanCommandBuffer::begin: unable to begin command buffer ({})",vk_code_to_string(code))),
        }
    }

    pub fn end(&self) -> Result<(),String> {

        match unsafe { ffi::vkEndCommandBuffer(self.vk_command_buffer) } {
            ffi::VK_SUCCESS => Ok(()),
            code => Err(format!("VulkanCommandBuffer::end: unable to end command buffer ({})",vk_code_to_string(code))),
        }
    }

    pub fn bind_compute_pipeline(&mut self,compute_pipeline: &Rc<ComputePipeline>) {

        self.compute_pipeline = Some(Rc::clone(&compute_pipeline));
        unsafe { ffi::vkCmdBindPipeline(
            self.vk_command_buffer,
            ffi::VK_PIPELINE_BIND_POINT_COMPUTE,
            compute_pipeline.vk_pipeline,
        ) };
    }

    pub fn bind_descriptor_set(&mut self,pipeline_layout: &Rc<PipelineLayout>,descriptor_set: &Rc<DescriptorSet>) {

        self.pipeline_layout = Some(Rc::clone(&pipeline_layout));
        self.descriptor_set = Some(Rc::clone(&descriptor_set));
        unsafe { ffi::vkCmdBindDescriptorSets(
            self.vk_command_buffer,
            ffi::VK_PIPELINE_BIND_POINT_COMPUTE,
            pipeline_layout.vk_pipeline_layout,
            0,
            1,
            &descriptor_set.vk_descriptor_set,
            0,
            null_mut()
        ) };
    }

    pub fn dispatch(&self,x: usize,y: usize,z: usize) {
        unsafe { ffi::vkCmdDispatch(self.vk_command_buffer,x as u32,y as u32,z as u32) };
    }
}

impl Drop for CommandBuffer {

    fn drop(&mut self) {
        unsafe { ffi::vkFreeCommandBuffers(self.gpu.vk_device,self.gpu.vk_command_pool,1,&self.vk_command_buffer) };
    }
}
