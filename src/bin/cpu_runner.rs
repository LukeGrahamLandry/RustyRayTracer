extern crate raytracer;
extern crate shaders;

use std::time::Instant;
use rayon::prelude::*;
use softbuffer::GraphicsContext;
use spirv_std::glam::{vec2, Vec2, vec4, Vec4};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use raytracer::timer::FrameTimer;
use shaders::{main_fs, ShaderConstants};


fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

    let start = Instant::now();
    let mut timer = FrameTimer::new();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
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

                        let frag_coord = (vec2(screen_pos.x, -screen_pos.y) + Vec2::ONE) / Vec2::splat(2.0)
                            * vec2(width as f32, height as f32);
                        let frag_coord = vec4(frag_coord.x, frag_coord.y, 0.0, 0.0);

                        let mut colour = vec4(0.0, 0.0, 0.0, 1.0);
                        main_fs(frag_coord, &push_constants, &mut colour);

                        color_u32_from_vec4(colour)
                    })
                    .collect::<Vec<_>>();

                graphics_context.set_buffer(&buffer, width as u16, height as u16);
                timer.update();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}

// https://github.com/EmbarkStudios/rust-gpu/blob/main/examples/runners/cpu/src/main.rs
fn srgb_oetf(x: f32) -> f32 {
    if x <= 0.0031308 {
        x * 12.92
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}

fn color_u32_from_vec4(v: Vec4) -> u32 {
    let convert = |f: f32| -> u32 { (f.clamp(0.0, 1.0) * 255.0).round() as u32 };

    convert(srgb_oetf(v.z))
        | convert(srgb_oetf(v.y)) << 8
        | convert(srgb_oetf(v.x)) << 16
        | convert(v.w) << 24
}