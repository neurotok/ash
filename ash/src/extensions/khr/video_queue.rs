use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Device, Entry, Instance};
use std::ffi::CStr;
use std::mem;
use std::ptr;

#[derive(Clone)]
pub struct VideoQueue {
    handle: vk::Device,
    fp: vk::KhrVideoQueueFn,
}

impl VideoQueue {
    pub fn new(entry: &Entry, instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrVideoQueueFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { handle, fp }
    }
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_video_capabilities(
        &self,
        pdevice: vk::PhysicalDevice,
        video_profile: &vk::VideoProfileInfoKHR,
        capabilities: &mut vk::VideoCapabilitiesKHR,
    ) -> VkResult<()> {
        (self.fp.get_physical_device_video_capabilities_khr)(pdevice, video_profile, capabilities)
            .result()
    }

    /// Retrieve the number of elements to pass to [`get_physical_device_video_format_properties_khr()`][Self::get_physical_device_video_format_properties_khr()]
    #[inline]
    pub unsafe fn get_physical_device_video_format_properties_len(
        &self,
        pdevice: vk::PhysicalDevice,
        video_format_info: &vk::PhysicalDeviceVideoFormatInfoKHR,
    ) -> usize {
        let mut properties_count = 0;
        let _r = (self.fp.get_physical_device_video_format_properties_khr)(
            pdevice,
            video_format_info,
            &mut properties_count,
            ptr::null_mut(),
        );
        properties_count as usize
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_video_format_properties(
        &self,
        pdevice: vk::PhysicalDevice,
        video_format_info: &vk::PhysicalDeviceVideoFormatInfoKHR,
        out: &mut [vk::VideoFormatPropertiesKHR],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        let result = (self.fp.get_physical_device_video_format_properties_khr)(
            pdevice,
            video_format_info,
            &mut count,
            out.as_mut_ptr(),
        )
        .result();
        assert_eq!(count as usize, out.len());
        result
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html>
    #[inline]
    pub unsafe fn create_video_session(
        &self,
        create_info: &vk::VideoSessionCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::VideoSessionKHR> {
        let mut video_session = mem::zeroed();
        (self.fp.create_video_session_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut video_session,
        )
        .result_with_success(video_session)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html>
    #[inline]
    pub unsafe fn destroy_video_session(
        &self,
        video_session: vk::VideoSessionKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp.destroy_video_session_khr)(
            self.handle,
            video_session,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html>
    #[inline]
    pub unsafe fn get_video_session_memory_requirements(
        &self,
        video_session: vk::VideoSessionKHR,
        out: &mut [vk::VideoSessionMemoryRequirementsKHR],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        let result = (self.fp.get_video_session_memory_requirements_khr)(
            self.handle,
            video_session,
            &mut count,
            out.as_mut_ptr(),
        )
        .result();
        assert_eq!(count as usize, out.len());
        result
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html>
    #[inline]
    pub unsafe fn bind_video_session_memory(
        &self,
        video_session: vk::VideoSessionKHR,
        out: &mut [vk::BindVideoSessionMemoryInfoKHR],
    ) -> VkResult<()> {
        let count = out.len() as u32;
        let result = (self.fp.bind_video_session_memory_khr)(
            self.handle,
            video_session,
            count,
            out.as_mut_ptr(),
        )
        .result();
        assert_eq!(count as usize, out.len());
        result
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html>
    #[inline]
    pub unsafe fn create_video_session_parameters(
        &self,
        create_info: &vk::VideoSessionParametersCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::VideoSessionParametersKHR> {
        let mut video_session = mem::zeroed();
        (self.fp.create_video_session_parameters_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut video_session,
        )
        .result_with_success(video_session)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html>
    #[inline]
    pub unsafe fn update_video_session_parameters(
        &self,
        video_session_parameters: vk::VideoSessionParametersKHR,
        memory_requirements: &vk::VideoSessionParametersUpdateInfoKHR,
    ) -> VkResult<()> {
        (self.fp.update_video_session_parameters_khr)(
            self.handle,
            video_session_parameters,
            memory_requirements,
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html>
    #[inline]
    pub unsafe fn destroy_video_session_parameters(
        &self,
        video_session_parameters: vk::VideoSessionParametersKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp.destroy_video_session_parameters_khr)(
            self.handle,
            video_session_parameters,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginVideoCodingKHR.html>
    #[inline]
    pub unsafe fn cmd_begin_video_coding(
        &self,
        command_buffer: vk::CommandBuffer,
        begin_info: &vk::VideoBeginCodingInfoKHR,
    ) {
        (self.fp.cmd_begin_video_coding_khr)(command_buffer, begin_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html>
    #[inline]
    pub unsafe fn cmd_end_video_coding(
        &self,
        command_buffer: vk::CommandBuffer,
        end_coding_info: &vk::VideoEndCodingInfoKHR,
    ) {
        (self.fp.cmd_end_video_coding_khr)(command_buffer, end_coding_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdControlVideoCodingKHR.html>
    #[inline]
    pub unsafe fn cmd_control_video_coding(
        &self,
        command_buffer: vk::CommandBuffer,
        coding_control_info: &vk::VideoCodingControlInfoKHR,
    ) {
        (self.fp.cmd_control_video_coding_khr)(command_buffer, coding_control_info)
    }

    #[inline]
    pub const fn name() -> &'static CStr {
        vk::KhrVideoQueueFn::name()
    }

    #[inline]
    pub fn fp(&self) -> &vk::KhrVideoQueueFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
