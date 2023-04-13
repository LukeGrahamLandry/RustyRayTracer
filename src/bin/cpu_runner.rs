use glam::{Vec4, vec4};
use rayon::iter::IntoParallelIterator;
use softbuffer::GraphicsContext;
use winit::dpi::LogicalSize;
use raytracer::shader_types::{PointLight, ShaderInputs, Shape};
use raytracer::window::{AppState, RenderStrategy};
use rayon::prelude::*;

fn main() {
    CpuState::run();
}

struct CpuState {
    graphics_context: GraphicsContext,
}

impl RenderStrategy for CpuState {
    fn new(app: &AppState) -> Self {
        CpuState {
            graphics_context: unsafe { GraphicsContext::new(&app.window, &app.window) }.unwrap()
        }
    }

    fn render(&mut self, app: &AppState) {
        let (width, height) = {
            let size = app.window.inner_size();
            (size.width, size.height)
        };

        let inputs = &app.shader_inputs();
        let shapes = app.world.get_shapes();
        let lights = app.world.get_lights();

        let buffer = (0..(width * height))
            .into_par_iter()
            .map(|i| {
                let x = i % width;
                let y = i / width;

                let colour = unsafe {trace_pixel(
                    vec4(x as f32, y as f32, 0.0, 0.0),
                    inputs as *const ShaderInputs,
                    shapes.as_ptr(),
                    lights.as_ptr()
                )};

                to_packed_colour(colour)
            })
            .collect::<Vec<_>>();

        self.graphics_context.set_buffer(&buffer, width as u16, height as u16);
    }

    fn resized(&mut self, _size: LogicalSize<u32>) {
        // NO-OP
    }

    fn world_changed(&mut self, _app: &AppState) {
        // NO-OP
    }
}

// the shader outputs colour as an rgb vector (0.0-1.0) but the screen wants a packed int with one byte for each component
fn to_packed_colour(v: Vec4) -> u32 {
    clamp_colour(v.z) | clamp_colour(v.y) << 8 | clamp_colour(v.x) << 16
}

fn clamp_colour(f: f32) -> u32 {
    (f.clamp(0.0, 1.0) * 255.0).round() as u32
}

extern {
    fn trace_pixel(position: Vec4, inputs: *const ShaderInputs, shapes: *const Shape, lights: *const PointLight) -> Vec4;
}