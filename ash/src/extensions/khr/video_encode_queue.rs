use crate::vk;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct VideoEncodeQueue {
    handle: vk::Instance,
    fp: vk::KhrVideoEncodeQueueFn,
}

impl VideoEncodeQueue {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::KhrVideoEncodeQueueFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.2-extensions/html/vkspec.html#vkCmdDecodeVideoKHR>
    #[inline]
    pub unsafe fn cmd_encode_video(
        &self,
        command_buffer: vk::CommandBuffer,
        encode_info: &vk::VideoEncodeInfoKHR,
    ) {
        (self.fp.cmd_encode_video_khr)(command_buffer, encode_info)
    }

    pub const NAME: &'static CStr = vk::KhrVideoEncodeQueueFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrVideoEncodeQueueFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
