extern crate raytracer;
extern crate shaders;

use std::time::Instant;

use rayon::prelude::*;
use raytracer::demo;
use softbuffer::GraphicsContext;
use spirv_std::glam::{vec4, Vec4};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use raytracer::timer::FrameTimer;
use shaders::{main_fs, ShaderInputs};

fn main() {
    println!("Loading...");
    let world = demo::chapter7();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rusty Raytracer (CPU)")
        .with_inner_size(winit::dpi::LogicalSize::new(
            world.camera.size().0,
            world.camera.size().1,
        ))
        .build(&event_loop)
        .unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

    let start = Instant::now();
    let mut timer = FrameTimer::new();

    println!("Begin event loop...");
    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let (width, height) = {
                let size = window.inner_size();
                (size.width, size.height)
            };

            let push_constants = ShaderInputs {
                time: start.elapsed().as_secs_f32(),
                camera: world.camera,
            };

            let buffer = (0..(width * height))
                .into_par_iter()
                .map(|i| {
                    let x = i % width;
                    let y = i / width;

                    let frag_coord = vec4(x as f32, y as f32, 0.0, 0.0);

                    let mut colour = vec4(0.0, 0.0, 0.0, 1.0);
                    main_fs(
                        frag_coord,
                        &push_constants,
                        &world.shapes,
                        &world.lights,
                        &mut colour,
                    );

                    to_packed_colour(colour)
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

// the shader outputs colour as an rgb vector (0.0-1.0) but the screen wants a packed int with one byte for each component
fn to_packed_colour(v: Vec4) -> u32 {
    clamp_colour(v.z) | clamp_colour(v.y) << 8 | clamp_colour(v.x) << 16
}

fn clamp_colour(f: f32) -> u32 {
    (f.clamp(0.0, 1.0) * 255.0).round() as u32
}
