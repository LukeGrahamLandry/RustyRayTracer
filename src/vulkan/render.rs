use ash::util::read_spv;
use ash::vk;

use std::io::Cursor;
use std::time::Instant;
use std::{collections::HashMap, default::Default, ffi::CString, ops::Drop};

use shaders::ShaderConstants;

use super::base::RenderBase;

pub struct RenderCtx {
    pub base: RenderBase,
    pub sync: RenderSync,

    pub swapchain: vk::SwapchainKHR,
    pub extent: vk::Extent2D,
    pub image_views: Vec<vk::ImageView>,
    pub render_pass: vk::RenderPass,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub commands: RenderCommandPool,
    pub viewports: Box<[vk::Viewport]>,
    pub scissors: Box<[vk::Rect2D]>,
    pub pipelines: Vec<Pipeline>,
    pub rendering_paused: bool,
    pub start: std::time::Instant,
    pub shader_module: vk::ShaderModule,
}

impl RenderCtx {
    pub fn from_base(base: RenderBase) -> Self {
        let sync = RenderSync::new(&base);

        let (swapchain, extent) = base.create_swapchain();
        let image_views = base.create_image_views(swapchain);
        let render_pass = base.create_render_pass();
        let framebuffers = base.create_framebuffers(&image_views, render_pass, extent);
        let commands = RenderCommandPool::new(&base);
        let (viewports, scissors) = {
            (
                Box::new([vk::Viewport {
                    x: 0.0,
                    y: extent.height as f32,
                    width: extent.width as f32,
                    height: -(extent.height as f32),
                    min_depth: 0.0,
                    max_depth: 1.0,
                }]),
                Box::new([vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent,
                }]),
            )
        };

        let spirv = read_spv(&mut Cursor::new(include_bytes!(env!("shaders.spv")))).unwrap();
        let shader_info = vk::ShaderModuleCreateInfo::builder().code(&spirv);
        let shader_module = unsafe {
            base.device
                .create_shader_module(&shader_info, None)
                .expect("Shader module error")
        };

        Self {
            sync,
            base,
            swapchain,
            extent,
            image_views,
            commands,
            render_pass,
            framebuffers,
            viewports,
            scissors,
            pipelines: Vec::new(),
            shader_module,
            rendering_paused: false,
            start: Instant::now(),
        }
    }

    pub fn create_pipeline_layout(&self) -> vk::PipelineLayout {
        let push_constant_range = vk::PushConstantRange::builder()
            .offset(0)
            .size(std::mem::size_of::<ShaderConstants>() as u32)
            .stage_flags(vk::ShaderStageFlags::ALL)
            .build();
        let layout_create_info = vk::PipelineLayoutCreateInfo::builder()
            .push_constant_ranges(&[push_constant_range])
            .build();
        unsafe {
            self.base
                .device
                .create_pipeline_layout(&layout_create_info, None)
                .unwrap()
        }
    }

    pub fn rebuild_pipelines(&mut self, pipeline_cache: vk::PipelineCache) {
        self.cleanup_pipelines();
        let pipeline_layout = self.create_pipeline_layout();
        let viewport = vk::PipelineViewportStateCreateInfo::builder()
            .scissor_count(1)
            .viewport_count(1);
        let modules_names = [()]
            .iter()
            .map(|_| {
                let vert_module = self.shader_module.clone();
                let vert_name = CString::new("main_vs").unwrap();
                let frag_module = self.shader_module.clone();
                let frag_name = CString::new("main_fs").unwrap();
                ((frag_module, frag_name), (vert_module, vert_name))
            })
            .collect::<Vec<_>>();
        let descs = modules_names
            .iter()
            .map(|((frag_module, frag_name), (vert_module, vert_name))| {
                PipelineDescriptor::new(Box::new([
                    vk::PipelineShaderStageCreateInfo {
                        module: *vert_module,
                        p_name: (*vert_name).as_ptr(),
                        stage: vk::ShaderStageFlags::VERTEX,
                        ..Default::default()
                    },
                    vk::PipelineShaderStageCreateInfo {
                        s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                        module: *frag_module,
                        p_name: (*frag_name).as_ptr(),
                        stage: vk::ShaderStageFlags::FRAGMENT,
                        ..Default::default()
                    },
                ]))
            })
            .collect::<Vec<_>>();
        let pipeline_info = descs
            .iter()
            .map(|desc| {
                vk::GraphicsPipelineCreateInfo::builder()
                    .stages(&desc.shader_stages)
                    .vertex_input_state(&desc.vertex_input)
                    .input_assembly_state(&desc.input_assembly)
                    .rasterization_state(&desc.rasterization)
                    .multisample_state(&desc.multisample)
                    .depth_stencil_state(&desc.depth_stencil)
                    .color_blend_state(&desc.color_blend)
                    .dynamic_state(&desc.dynamic_state_info)
                    .viewport_state(&viewport)
                    .layout(pipeline_layout)
                    .render_pass(self.render_pass)
                    .build()
            })
            .collect::<Vec<_>>();
        self.pipelines = unsafe {
            self.base
                .device
                .create_graphics_pipelines(pipeline_cache, &pipeline_info, None)
                .expect("Unable to create graphics pipeline")
        }
        .iter()
        .zip(descs)
        .map(|(&pipeline, desc)| Pipeline {
            pipeline,
            pipeline_layout,
            color_blend_attachments: desc.color_blend_attachments,
            dynamic_state: desc.dynamic_state,
        })
        .collect();
    }

    pub fn cleanup_pipelines(&mut self) {
        unsafe {
            self.base.device.device_wait_idle().unwrap();
            for pipeline in self.pipelines.drain(..) {
                self.base.device.destroy_pipeline(pipeline.pipeline, None);
                self.base
                    .device
                    .destroy_pipeline_layout(pipeline.pipeline_layout, None);
            }
        }
    }

    pub fn build_pipelines(&mut self, pipeline_cache: vk::PipelineCache) {
        self.rebuild_pipelines(pipeline_cache);
    }

    /// Destroys the swapchain, as well as the renderpass and frame and command buffers
    pub fn cleanup_swapchain(&mut self) {
        unsafe {
            self.base.device.device_wait_idle().unwrap();
            // framebuffers
            for framebuffer in self.framebuffers.drain(..) {
                self.base.device.destroy_framebuffer(framebuffer, None);
            }
            // image views
            for image_view in self.image_views.drain(..) {
                self.base.device.destroy_image_view(image_view, None);
            }
            // swapchain
            self.base
                .swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
    }

    /// Recreates the swapchain, but does not recreate the pipelines because they use dynamic state.
    pub fn recreate_swapchain(&mut self) {
        let surface_resolution = self.base.surface_resolution();

        if surface_resolution.width == 0 || surface_resolution.height == 0 {
            self.rendering_paused = true;
            return;
        } else if self.rendering_paused {
            self.rendering_paused = false;
        };

        self.cleanup_swapchain();

        let (swapchain, extent) = self.base.create_swapchain();
        self.swapchain = swapchain;
        self.extent = extent;
        self.image_views = self.base.create_image_views(self.swapchain);
        self.framebuffers =
            self.base
                .create_framebuffers(&self.image_views, self.render_pass, extent);
        self.viewports = Box::new([vk::Viewport {
            x: 0.0,
            y: extent.height as f32,
            width: extent.width as f32,
            height: -(extent.height as f32),
            min_depth: 0.0,
            max_depth: 1.0,
        }]);
        self.scissors = Box::new([vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent,
        }]);
    }

    pub fn render(&mut self) {
        let present_index = unsafe {
            match self.base.swapchain_loader.acquire_next_image(
                self.swapchain,
                std::u64::MAX,
                self.sync.present_complete_semaphore,
                vk::Fence::null(),
            ) {
                Ok((idx, _)) => idx,
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                    self.recreate_swapchain();
                    return;
                }
                Err(err) => panic!("failed to acquire next image: {err:?}"),
            }
        };

        let framebuffer = self.framebuffers[present_index as usize];
        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 1.0, 0.0],
            },
        }];

        // There should only be one pipeline because compile_shaders only loads the last spirv
        // file it produced.
        for pipeline in self.pipelines.iter() {
            self.draw(pipeline, framebuffer, &clear_values);
        }

        let wait_semaphors = [self.sync.rendering_complete_semaphore];
        let swapchains = [self.swapchain];
        let image_indices = [present_index];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphors)
            .swapchains(&swapchains)
            .image_indices(&image_indices);
        unsafe {
            match self
                .base
                .swapchain_loader
                .queue_present(self.base.present_queue, &present_info)
            {
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) | Ok(true) => self.recreate_swapchain(),
                Ok(false) => {}
                Err(err) => panic!("failed to present queue: {err:?}"),
            };
        }
    }

    pub fn draw(
        &self,
        pipeline: &Pipeline,
        framebuffer: vk::Framebuffer,
        clear_values: &[vk::ClearValue],
    ) {
        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass)
            .framebuffer(framebuffer)
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.scissors[0].extent,
            })
            .clear_values(clear_values)
            .build();
        self.record_submit_commandbuffer(
            &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
            |device, draw_command_buffer| unsafe {
                device.cmd_begin_render_pass(
                    draw_command_buffer,
                    &render_pass_begin_info,
                    vk::SubpassContents::INLINE,
                );
                device.cmd_bind_pipeline(
                    draw_command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    pipeline.pipeline,
                );
                device.cmd_set_viewport(draw_command_buffer, 0, &self.viewports);
                device.cmd_set_scissor(draw_command_buffer, 0, &self.scissors);

                let push_constants = ShaderConstants {
                    time: self.start.elapsed().as_secs_f32(),
                };
                device.cmd_push_constants(
                    draw_command_buffer,
                    pipeline.pipeline_layout,
                    ash::vk::ShaderStageFlags::ALL,
                    0,
                    any_as_u8_slice(&push_constants),
                );

                device.cmd_draw(draw_command_buffer, 3, 1, 0, 0);
                device.cmd_end_render_pass(draw_command_buffer);
            },
        );
    }

    /// Helper function for submitting command buffers. Immediately waits for the fence before the command buffer
    /// is executed. That way we can delay the waiting for the fences by 1 frame which is good for performance.
    /// Make sure to create the fence in a signaled state on the first use.
    pub fn record_submit_commandbuffer<F: FnOnce(&ash::Device, vk::CommandBuffer)>(
        &self,
        wait_mask: &[vk::PipelineStageFlags],
        f: F,
    ) {
        unsafe {
            self.base
                .device
                .wait_for_fences(&[self.sync.draw_commands_reuse_fence], true, std::u64::MAX)
                .expect("Wait for fence failed.");

            self.base
                .device
                .reset_fences(&[self.sync.draw_commands_reuse_fence])
                .expect("Reset fences failed.");

            // As we only have a single command buffer, we can simply reset the entire pool instead of just the buffer.
            // Doing this is a little bit faster, see
            // https://arm-software.github.io/vulkan_best_practice_for_mobile_developers/samples/performance/command_buffer_usage/command_buffer_usage_tutorial.html#resetting-the-command-pool
            self.base
                .device
                .reset_command_pool(self.commands.pool, vk::CommandPoolResetFlags::empty())
                .expect("Reset command pool failed.");

            let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

            self.base
                .device
                .begin_command_buffer(
                    self.commands.draw_command_buffer,
                    &command_buffer_begin_info,
                )
                .expect("Begin commandbuffer");

            f(&self.base.device, self.commands.draw_command_buffer);

            self.base
                .device
                .end_command_buffer(self.commands.draw_command_buffer)
                .expect("End commandbuffer");

            let command_buffers = vec![self.commands.draw_command_buffer];
            let wait_semaphores = &[self.sync.present_complete_semaphore];
            let signal_semaphores = &[self.sync.rendering_complete_semaphore];
            let submit_info = vk::SubmitInfo::builder()
                .wait_semaphores(wait_semaphores)
                .wait_dst_stage_mask(wait_mask)
                .command_buffers(&command_buffers)
                .signal_semaphores(signal_semaphores);

            self.base
                .device
                .queue_submit(
                    self.base.present_queue,
                    &[submit_info.build()],
                    self.sync.draw_commands_reuse_fence,
                )
                .expect("queue submit failed.");
        }
    }
}

impl Drop for RenderCtx {
    fn drop(&mut self) {
        unsafe {
            self.base.device.device_wait_idle().unwrap();
            self.base
                .device
                .destroy_semaphore(self.sync.present_complete_semaphore, None);
            self.base
                .device
                .destroy_semaphore(self.sync.rendering_complete_semaphore, None);
            self.base
                .device
                .destroy_fence(self.sync.draw_commands_reuse_fence, None);
            self.base
                .device
                .free_command_buffers(self.commands.pool, &[self.commands.draw_command_buffer]);
            self.base.device.destroy_render_pass(self.render_pass, None);
            self.cleanup_pipelines();
            self.cleanup_swapchain();
            self.base
                .device
                .destroy_command_pool(self.commands.pool, None);
            self.base
                .device
                .destroy_shader_module(self.shader_module, None);
        }
    }
}

pub struct RenderSync {
    pub present_complete_semaphore: vk::Semaphore,
    pub rendering_complete_semaphore: vk::Semaphore,
    pub draw_commands_reuse_fence: vk::Fence,
}

impl RenderSync {
    pub fn new(base: &RenderBase) -> Self {
        let fence_create_info =
            vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

        let semaphore_create_info = vk::SemaphoreCreateInfo::default();

        unsafe {
            let draw_commands_reuse_fence = base
                .device
                .create_fence(&fence_create_info, None)
                .expect("Create fence failed.");
            let present_complete_semaphore = base
                .device
                .create_semaphore(&semaphore_create_info, None)
                .unwrap();
            let rendering_complete_semaphore = base
                .device
                .create_semaphore(&semaphore_create_info, None)
                .unwrap();

            Self {
                present_complete_semaphore,
                rendering_complete_semaphore,
                draw_commands_reuse_fence,
            }
        }
    }
}

pub struct RenderCommandPool {
    pub pool: vk::CommandPool,
    pub draw_command_buffer: vk::CommandBuffer,
}

impl RenderCommandPool {
    pub fn new(base: &RenderBase) -> Self {
        let pool = {
            let pool_create_info =
                vk::CommandPoolCreateInfo::builder().queue_family_index(base.queue_family_index);

            unsafe {
                base.device
                    .create_command_pool(&pool_create_info, None)
                    .unwrap()
            }
        };

        let command_buffers = {
            let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_buffer_count(1)
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY);

            unsafe {
                base.device
                    .allocate_command_buffers(&command_buffer_allocate_info)
                    .unwrap()
            }
        };

        Self {
            pool,
            draw_command_buffer: command_buffers[0],
        }
    }
}

pub struct Pipeline {
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
    pub color_blend_attachments: Box<[vk::PipelineColorBlendAttachmentState]>,
    pub dynamic_state: Box<[vk::DynamicState]>,
}

pub struct PipelineDescriptor {
    pub color_blend_attachments: Box<[vk::PipelineColorBlendAttachmentState]>,
    pub dynamic_state: Box<[vk::DynamicState]>,
    pub shader_stages: Box<[vk::PipelineShaderStageCreateInfo]>,
    pub vertex_input: vk::PipelineVertexInputStateCreateInfo,
    pub input_assembly: vk::PipelineInputAssemblyStateCreateInfo,
    pub rasterization: vk::PipelineRasterizationStateCreateInfo,
    pub multisample: vk::PipelineMultisampleStateCreateInfo,
    pub depth_stencil: vk::PipelineDepthStencilStateCreateInfo,
    pub color_blend: vk::PipelineColorBlendStateCreateInfo,
    pub dynamic_state_info: vk::PipelineDynamicStateCreateInfo,
}

impl PipelineDescriptor {
    fn new(shader_stages: Box<[vk::PipelineShaderStageCreateInfo]>) -> Self {
        let vertex_input = vk::PipelineVertexInputStateCreateInfo {
            vertex_attribute_description_count: 0,
            vertex_binding_description_count: 0,
            ..Default::default()
        };
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };

        let rasterization = vk::PipelineRasterizationStateCreateInfo {
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            line_width: 1.0,
            polygon_mode: vk::PolygonMode::FILL,
            ..Default::default()
        };
        let multisample = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            ..Default::default()
        };
        let noop_stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            ..Default::default()
        };
        let depth_stencil = vk::PipelineDepthStencilStateCreateInfo {
            depth_test_enable: 0,
            depth_write_enable: 0,
            depth_compare_op: vk::CompareOp::ALWAYS,
            front: noop_stencil_state,
            back: noop_stencil_state,
            max_depth_bounds: 1.0,
            ..Default::default()
        };
        let color_blend_attachments = Box::new([vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ZERO,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
        }]);
        let color_blend = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(color_blend_attachments.as_ref())
            .build();

        let dynamic_state = Box::new([vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR]);
        let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::builder()
            .dynamic_states(dynamic_state.as_ref())
            .build();

        Self {
            color_blend_attachments,
            dynamic_state,
            shader_stages,
            vertex_input,
            input_assembly,
            rasterization,
            multisample,
            depth_stencil,
            color_blend,
            dynamic_state_info,
        }
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T).cast::<u8>(), ::std::mem::size_of::<T>())
}
