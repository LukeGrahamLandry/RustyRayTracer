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
    screen_buffer: Vec<u32>
}

impl RenderStrategy for CpuState {
    fn new(app: &AppState) -> Self {
        println!("Shaders will (slowly) run on the CPU.");
        CpuState {
            graphics_context: unsafe { GraphicsContext::new(&app.window, &app.window) }.unwrap(),
            screen_buffer: vec![]
        }
    }

    fn render(&mut self, app: &AppState) {
        let (width, height) = (app.window.inner_size().width, app.window.inner_size().height);

        let inputs = &app.shader_inputs();
        let shapes = app.world.get_shapes();
        let lights = app.world.get_lights();

        (0..(width * height))
            .into_par_iter()
            .map(|i| {
                let x = i % width;
                let y = i / width;

                let mut colour = Vec4::ZERO;
                unsafe {trace_pixel(
                    vec4(x as f32, y as f32, 0.0, 0.0),
                    inputs,
                    shapes.as_ptr(),
                    lights.as_ptr(),
                    &mut colour
                )};

                to_packed_colour(colour)
            })
            .collect_into_vec(&mut self.screen_buffer);

        self.graphics_context.set_buffer(&self.screen_buffer, width as u16, height as u16);
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
    fn trace_pixel(position: Vec4, inputs: *const ShaderInputs, shapes: *const Shape, lights: *const PointLight, out: *mut Vec4);
}
