use crate::vk;
use crate::{Device, Entry, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct VideoEncodeQueue {
    handle: vk::Device,
    fp: vk::KhrVideoEncodeQueueFn,
}

impl VideoEncodeQueue {
    pub fn new(entry: &Entry, instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrVideoEncodeQueueFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
            //mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
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

    #[inline]
    pub const fn name() -> &'static CStr {
        vk::KhrVideoEncodeQueueFn::name()
    }

    #[inline]
    pub fn fp(&self) -> &vk::KhrVideoEncodeQueueFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
