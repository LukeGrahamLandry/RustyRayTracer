use ash::{
    extensions::{ext, khr},
    vk,
};

use gpu_allocator::vulkan::*;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use std::{
    borrow::Cow,
    default::Default,
    ffi::{CStr, CString},
    ops::Drop,
};

pub const ENABLE_DEBUG_LAYER: bool = false;

use super::render::{RenderCtx, RenderSync};
pub struct RenderBase {
    pub allocator: Allocator,

    pub swapchain_loader: khr::Swapchain,

    pub debug_utils_loader: Option<ext::DebugUtils>,
    pub debug_call_back: Option<vk::DebugUtilsMessengerEXT>,

    pub pdevice: vk::PhysicalDevice,
    pub queue_family_index: u32,
    pub present_queue: vk::Queue,

    pub surface: vk::SurfaceKHR,
    pub surface_loader: khr::Surface,
    pub surface_format: vk::SurfaceFormatKHR,

    pub device: ash::Device,
    pub instance: ash::Instance,
    pub entry: ash::Entry,
    pub window: winit::window::Window,
}

impl RenderBase {
    pub fn new(window: winit::window::Window) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "macos")] {
                let entry = ash_molten::load();
            } else {
                let entry = unsafe{ash::Entry::load()}.unwrap();
            }
        }

        let instance: ash::Instance = {
            let app_name = CString::new("VulkanTriangle").unwrap();

            let layer_names = if ENABLE_DEBUG_LAYER {
                vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]
            } else {
                vec![]
            };
            let layers_names_raw: Vec<*const i8> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let mut extension_names_raw =
                ash_window::enumerate_required_extensions(window.raw_display_handle())
                    .unwrap()
                    .to_vec();
            if ENABLE_DEBUG_LAYER {
                extension_names_raw.push(ext::DebugUtils::name().as_ptr());
            }

            let appinfo = vk::ApplicationInfo::builder()
                .application_name(&app_name)
                .application_version(0)
                .engine_name(&app_name)
                .engine_version(0)
                .api_version(vk::make_api_version(0, 1, 2, 0));

            let instance_create_info = vk::InstanceCreateInfo::builder()
                .application_info(&appinfo)
                .enabled_layer_names(&layers_names_raw)
                .enabled_extension_names(&extension_names_raw);

            unsafe {
                entry
                    .create_instance(&instance_create_info, None)
                    .expect("Instance creation error")
            }
        };

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
            .unwrap()
        };

        let (debug_utils_loader, debug_call_back) = if ENABLE_DEBUG_LAYER {
            let debug_utils_loader = ext::DebugUtils::new(&entry, &instance);
            let debug_call_back = {
                let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
                    .message_severity(
                        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                            | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                            | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                    )
                    .message_type(
                        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                    )
                    .pfn_user_callback(Some(vulkan_debug_callback));

                unsafe {
                    debug_utils_loader
                        .create_debug_utils_messenger(&debug_info, None)
                        .unwrap()
                }
            };

            (Some(debug_utils_loader), Some(debug_call_back))
        } else {
            (None, None)
        };

        let surface_loader = khr::Surface::new(&entry, &instance);

        let (pdevice, queue_family_index) = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Physical device error")
                .iter()
                .find_map(|pdevice| {
                    instance
                        .get_physical_device_queue_family_properties(*pdevice)
                        .iter()
                        .enumerate()
                        .find_map(|(index, info)| {
                            if info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                                && surface_loader
                                    .get_physical_device_surface_support(
                                        *pdevice,
                                        index as u32,
                                        surface,
                                    )
                                    .unwrap()
                            {
                                Some((*pdevice, index as u32))
                            } else {
                                None
                            }
                        })
                })
                .expect("Couldn't find suitable device.")
        };

        let device: ash::Device = {
            let device_extension_names_raw = [khr::Swapchain::name().as_ptr()];
            let features = vk::PhysicalDeviceFeatures {
                shader_clip_distance: 1,
                ..Default::default()
            };
            let priorities = [1.0];
            let queue_info = [vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family_index)
                .queue_priorities(&priorities)
                .build()];

            let mut vulkan_memory_model_features =
                vk::PhysicalDeviceVulkanMemoryModelFeatures::builder()
                    .vulkan_memory_model(true)
                    .build();

            let device_create_info = vk::DeviceCreateInfo::builder()
                .push_next(&mut vulkan_memory_model_features)
                .queue_create_infos(&queue_info)
                .enabled_extension_names(&device_extension_names_raw)
                .enabled_features(&features);
            unsafe {
                instance
                    .create_device(pdevice, &device_create_info, None)
                    .unwrap()
            }
        };

        let swapchain_loader = khr::Swapchain::new(&instance, &device);

        let present_queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        let surface_format = {
            let acceptable_formats = {
                [
                    vk::Format::R8G8B8_SRGB,
                    vk::Format::B8G8R8_SRGB,
                    vk::Format::R8G8B8A8_SRGB,
                    vk::Format::B8G8R8A8_SRGB,
                    vk::Format::A8B8G8R8_SRGB_PACK32,
                ]
            };
            unsafe {
                *surface_loader
                    .get_physical_device_surface_formats(pdevice, surface)
                    .unwrap()
                    .iter()
                    .find(|sfmt| acceptable_formats.contains(&sfmt.format))
                    .expect("Unable to find suitable surface format.")
            }
        };

        let allocator = Allocator::new(&AllocatorCreateDesc {
            instance: instance.clone(),
            device: device.clone(),
            physical_device: pdevice,
            debug_settings: Default::default(),
            buffer_device_address: true,
        })
        .unwrap();

        Self {
            window,
            entry,
            instance,
            device,
            swapchain_loader,
            debug_utils_loader,
            debug_call_back,
            pdevice,
            queue_family_index,
            present_queue,
            surface,
            surface_loader,
            surface_format,
            allocator,
        }
    }

    pub fn surface_resolution(&self) -> vk::Extent2D {
        let surface_capabilities = self.surface_capabilities();
        match surface_capabilities.current_extent.width {
            std::u32::MAX => {
                let window_inner = self.window.inner_size();
                vk::Extent2D {
                    width: window_inner.width,
                    height: window_inner.height,
                }
            }
            _ => surface_capabilities.current_extent,
        }
    }

    pub fn surface_capabilities(&self) -> vk::SurfaceCapabilitiesKHR {
        unsafe {
            self.surface_loader
                .get_physical_device_surface_capabilities(self.pdevice, self.surface)
                .unwrap()
        }
    }

    pub fn create_swapchain(&self) -> (vk::SwapchainKHR, vk::Extent2D, u32) {
        let surface_capabilities = self.surface_capabilities();
        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0
            && desired_image_count > surface_capabilities.max_image_count
        {
            desired_image_count = surface_capabilities.max_image_count;
        }
        let pre_transform = if surface_capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };
        let present_mode = unsafe {
            self.surface_loader
                .get_physical_device_surface_present_modes(self.pdevice, self.surface)
                .unwrap()
                .iter()
                .cloned()
                .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
                .unwrap_or(vk::PresentModeKHR::FIFO)
        };
        let extent = self.surface_resolution();
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(self.surface)
            .min_image_count(desired_image_count)
            .image_color_space(self.surface_format.color_space)
            .image_format(self.surface_format.format)
            .image_extent(extent)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);
        let swapchain = unsafe {
            self.swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .unwrap()
        };
        (swapchain, extent, desired_image_count)
    }

    pub fn create_image_views(&self, swapchain: vk::SwapchainKHR) -> Vec<vk::ImageView> {
        unsafe {
            self.swapchain_loader
                .get_swapchain_images(swapchain)
                .unwrap()
                .iter()
                .map(|&image| {
                    let create_view_info = vk::ImageViewCreateInfo::builder()
                        .view_type(vk::ImageViewType::TYPE_2D)
                        .format(self.surface_format.format)
                        .components(vk::ComponentMapping {
                            r: vk::ComponentSwizzle::R,
                            g: vk::ComponentSwizzle::G,
                            b: vk::ComponentSwizzle::B,
                            a: vk::ComponentSwizzle::A,
                        })
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        })
                        .image(image);
                    self.device
                        .create_image_view(&create_view_info, None)
                        .unwrap()
                })
                .collect()
        }
    }

    pub fn create_framebuffers(
        &self,
        image_views: &[vk::ImageView],
        render_pass: vk::RenderPass,
        extent: vk::Extent2D,
    ) -> Vec<vk::Framebuffer> {
        image_views
            .iter()
            .map(|&present_image_view| {
                let framebuffer_attachments = [present_image_view];
                unsafe {
                    self.device
                        .create_framebuffer(
                            &vk::FramebufferCreateInfo::builder()
                                .render_pass(render_pass)
                                .attachments(&framebuffer_attachments)
                                .width(extent.width)
                                .height(extent.height)
                                .layers(1),
                            None,
                        )
                        .unwrap()
                }
            })
            .collect()
    }

    pub fn create_render_pass(&self) -> vk::RenderPass {
        let renderpass_attachments = [vk::AttachmentDescription {
            format: self.surface_format.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        }];
        let color_attachment_refs = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];
        let dependencies = [vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            ..Default::default()
        }];
        let subpasses = [vk::SubpassDescription::builder()
            .color_attachments(&color_attachment_refs)
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .build()];
        let renderpass_create_info = vk::RenderPassCreateInfo::builder()
            .attachments(&renderpass_attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);
        unsafe {
            self.device
                .create_render_pass(&renderpass_create_info, None)
                .unwrap()
        }
    }

    pub fn create_render_sync(&self) -> RenderSync {
        RenderSync::new(self)
    }

    pub fn into_ctx(self) -> RenderCtx {
        RenderCtx::from_base(self)
    }

    pub fn create_descriptor_layout(&self) -> vk::DescriptorSetLayout {
        let descriptorset_layout_binding_descs = [
            vk::DescriptorSetLayoutBinding::builder()
                .binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
                .build(),
            vk::DescriptorSetLayoutBinding::builder()
                .binding(1)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
                .build(),
        ];
        let descriptorset_layout_info = vk::DescriptorSetLayoutCreateInfo::builder()
            .bindings(&descriptorset_layout_binding_descs);

        unsafe {
            self.device
                .create_descriptor_set_layout(&descriptorset_layout_info, None)
                .unwrap()
        }
    }
}

impl Drop for RenderBase {
    fn drop(&mut self) {
        println!("Dropping RenderBase...");
        unsafe {
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            if let Some((debug_utils, call_back)) =
                Option::zip(self.debug_utils_loader.take(), self.debug_call_back.take())
            {
                debug_utils.destroy_debug_utils_messenger(call_back, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{:?}:\n{:?} [{} ({})] : {}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    vk::FALSE
}
