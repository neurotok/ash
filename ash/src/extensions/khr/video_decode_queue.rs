use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;
/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_decode_queue.html>
#[derive(Clone)]
pub struct VideoDecodeQueue {
    handle: vk::Device,
    fp: vk::KhrVideoDecodeQueueFn,
}

impl VideoDecodeQueue {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrVideoDecodeQueueFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.2-extensions/html/vkspec.html#vkCmdDecodeVideoKHR>
    #[inline]
    pub unsafe fn cmd_decode_video(
        &self,
        command_buffer: vk::CommandBuffer,
        decode_info: &vk::VideoDecodeInfoKHR,
    ) {
        (self.fp.cmd_decode_video_khr)(command_buffer, decode_info)
    }

    pub const NAME: &'static CStr = vk::KhrVideoDecodeQueueFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrVideoDecodeQueueFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
