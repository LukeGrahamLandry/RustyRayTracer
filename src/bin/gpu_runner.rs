extern crate raytracer;

use ash::vk;

use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use raytracer::timer::FrameTimer;
use raytracer::vulkan::base::RenderBase;
use raytracer::{demo};

pub fn main() {
    println!("Loading...");
    let world = demo::chapter7();
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Rusty Raytracer (GPU)")
        .with_inner_size(winit::dpi::LogicalSize::new(
            world.camera.size().0,
            world.camera.size().1,
        ))
        .build(&event_loop)
        .unwrap();
    let mut ctx = RenderBase::new(window).into_ctx();

    ctx.build_pipelines(vk::PipelineCache::null());

    let shapes = ctx.allocate_buffer(&world.shapes);
    let lights = ctx.allocate_buffer(&world.lights);
    ctx.update_descriptor_set(shapes, lights);
    ctx.camera = world.camera;

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
