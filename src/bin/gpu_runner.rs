extern crate raytracer;

use ash::vk;

use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use raytracer::vulkan::base::RenderBase;
use raytracer::{scene::create_shapes, timer::FrameTimer};

pub fn main() {
    println!("Loading...");
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Rusty Raytracer (GPU)")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();
    let mut ctx = RenderBase::new(window).into_ctx();

    // Create shader module and pipelines
    ctx.build_pipelines(vk::PipelineCache::null());

    let (allocation, buffer) = ctx.create_shapes_buffer(&create_shapes());
    ctx.update_descriptor_set((allocation, buffer));

    let mut timer = FrameTimer::new();

    println!("Begin event loop...");
    event_loop.run(move |event, _window_target, control_flow| match event {
        Event::RedrawEventsCleared { .. } => {
            if ctx.rendering_paused {
                let vk::Extent2D { width, height } = ctx.base.surface_resolution();
                if height > 0 && width > 0 {
                    ctx.recreate_swapchain();
                    ctx.render();
                }
            } else {
                ctx.render();
                timer.update();
            }
        }
        Event::MainEventsCleared => {
            ctx.base.window.request_redraw();
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            WindowEvent::Resized(_) => {
                ctx.recreate_swapchain();
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
