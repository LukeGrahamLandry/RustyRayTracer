extern crate raytracer;
extern crate shaders;

use std::time::Instant;

use rayon::prelude::*;
use softbuffer::GraphicsContext;
use spirv_std::glam::{vec2, vec4, Vec2, Vec4};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use raytracer::scene::World;
use raytracer::timer::FrameTimer;
use raytracer::vulkan::color_u32_from_vec4;
use shaders::{main_fs, ShaderConstants};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rusty Raytracer (CPU)")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

    let start = Instant::now();
    let mut timer = FrameTimer::new();
    let world = World::default();
    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let (width, height) = {
                let size = window.inner_size();
                (size.width, size.height)
            };

            let push_constants = ShaderConstants {
                time: start.elapsed().as_secs_f32(),
            };

            let buffer = (0..(width * height))
                .into_par_iter()
                .map(|i| {
                    let screen_pos = vec2(
                        (i % width) as f32 / width as f32 * 2.0 - 1.0,
                        -((i / width) as f32 / height as f32 * 2.0 - 1.0),
                    );

                    let frag_coord = (vec2(screen_pos.x, -screen_pos.y) + Vec2::ONE)
                        / Vec2::splat(2.0)
                        * vec2(width as f32, height as f32);
                    let frag_coord = vec4(frag_coord.x, frag_coord.y, 0.0, 0.0);

                    let mut colour = vec4(0.0, 0.0, 0.0, 1.0);
                    main_fs(
                        frag_coord,
                        &push_constants,
                        &world.shapes,
                        &world.lights,
                        &mut colour,
                    );

                    color_u32_from_vec4(colour)
                })
                .collect::<Vec<_>>();

            graphics_context.set_buffer(&buffer, width as u16, height as u16);
            timer.update();
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
