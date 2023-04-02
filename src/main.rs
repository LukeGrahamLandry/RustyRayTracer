use std::rc::Rc;
use wgpu::{RenderPipeline, SurfaceError};
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, WindowEvent};
use crate::window::{App, WindowContext};

mod window;

fn main() {
    env_logger::init();
    pollster::block_on(WindowContext::run(AppState::new));
}

struct AppState {
    ctx: Rc<WindowContext>,
    render_pipeline: RenderPipeline,
}

impl App for AppState {
    fn new(ctx: Rc<WindowContext>) -> Self {
        let render_pipeline = ctx.render_pipeline();
        AppState {
            ctx,
            render_pipeline
        }
    }

    fn render(&mut self) -> Result<(), SurfaceError> {
        let mut encoder = self.ctx.command_encoder();
        let output = self.ctx.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass = self.ctx.render_pass(&mut encoder, &view);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        self.ctx.queue.submit(Some(encoder.finish()));
        output.present();
        Ok(())
    }

    fn handle_window_event(&mut self, _event: &WindowEvent) {

    }

    fn handle_device_event(&mut self, _event: &DeviceEvent) {

    }

    fn resize(&mut self, _new_size: PhysicalSize<u32>) {

    }
}
