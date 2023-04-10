use std::ffi::c_void;
use std::mem::size_of;
use std::time::Instant;
use metal::*;
use crate::gpu::setup::AppState;
use crate::types::ShaderInputs;

impl AppState {
    pub(crate) fn render(&self) {
        let drawable = match self.layer.next_drawable() {
            Some(drawable) => drawable,
            None => return,
        };

        let pass_descriptor = RenderPassDescriptor::new();
        init_pass(&pass_descriptor, drawable.texture());

        let command_buffer = self.command_queue.new_command_buffer();
        let encoder =
            command_buffer.new_render_command_encoder(&pass_descriptor);

        encoder.set_render_pipeline_state(&self.pipeline_state);
        let inputs = ShaderInputs {
            time: (Instant::now() - self.start).as_secs_f32(),
            camera: self.world.camera,
            shape_count: self.world.shapes.len() as u32,
            light_count: self.world.lights.len() as u32,
        };
        encoder.set_fragment_bytes(0, size_of::<ShaderInputs>() as u64, ptr(&inputs));
        encoder.set_fragment_buffer(1, Some(&self.shapes_buffer), 0);
        encoder.set_fragment_buffer(2, Some(&self.lights_buffer), 0);
        encoder.draw_primitives(
            MTLPrimitiveType::Triangle,
            0,
            3,
        );

        encoder.end_encoding();
        command_buffer.present_drawable(&drawable);
        command_buffer.commit();
    }
}

fn init_pass(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();
    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.0, 0.0, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

fn ptr<T>(data: &T) -> *const c_void {
    data as *const T as *const c_void
}