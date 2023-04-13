extern crate objc;
extern crate raytracer;

use std::mem;
use std::ffi::c_void;

use metal::*;
use winit::dpi::LogicalSize;
use winit::platform::macos::WindowExtMacOS;
use core_graphics_types::geometry::CGSize;
use objc::{rc::autoreleasepool, runtime::YES};
use cocoa::{appkit::NSView, base::id as cocoa_id};

use raytracer::window::{RenderStrategy, AppState};
use raytracer::shader_types::ShaderInputs;

pub fn main() {
    GpuState::run();
}

struct GpuState {
    layer: MetalLayer,
    command_queue: CommandQueue,
    pipeline_state: RenderPipelineState,
    shapes_buffer: Buffer,
    lights_buffer: Buffer,
    device: Device,
}

impl RenderStrategy for GpuState {
    fn new(app: &AppState) -> GpuState {
        println!("Shaders will run on the GPU.");
        let device = Device::system_default().expect("No metal device found.");
        let layer = init_layer(&device, app);
        init_view(app, &layer);
        GpuState {
            layer,
            pipeline_state: init_pipeline(&device, load_shaders(&device)),
            command_queue: device.new_command_queue(),
            shapes_buffer: init_buffer(&device, app.world.get_shapes()),
            lights_buffer: init_buffer(&device, app.world.get_lights()),
            device
        }
    }

    fn render(&mut self, app: &AppState) {
        autoreleasepool(|| self.do_render(app));
    }

    fn resized(&mut self, size: LogicalSize<u32>) {
        self.layer.set_drawable_size(CGSize::new(size.width as f64, size.height as f64));
    }

    fn world_changed(&mut self, app: &AppState) {
        self.shapes_buffer = init_buffer(&self.device, app.world.get_shapes());
        self.lights_buffer = init_buffer(&self.device, app.world.get_lights());
    }
}

impl GpuState {
    fn do_render(&mut self, app: &AppState) {
        let drawable = self.layer.next_drawable().unwrap();
        let pass_descriptor = RenderPassDescriptor::new();
        init_pass(&pass_descriptor, drawable.texture());
        let command_buffer = self.command_queue.new_command_buffer();
        let encoder = command_buffer.new_render_command_encoder(&pass_descriptor);

        encoder.set_render_pipeline_state(&self.pipeline_state);
        self.set_buffers(app, encoder);
        encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);

        encoder.end_encoding();
        command_buffer.present_drawable(&drawable);
        command_buffer.commit();
    }

    fn set_buffers(&self, app: &AppState, encoder: &RenderCommandEncoderRef) {
        encoder.set_fragment_bytes(0, mem::size_of::<ShaderInputs>() as u64, ptr(&app.shader_inputs()));
        encoder.set_fragment_buffer(1, Some(&self.shapes_buffer), 0);
        encoder.set_fragment_buffer(2, Some(&self.lights_buffer), 0);
    }
}

fn init_pipeline(device: &Device, shaders: (Function, Function)) -> RenderPipelineState {
    let pipeline_state_descriptor = RenderPipelineDescriptor::new();
    pipeline_state_descriptor.set_vertex_function(Some(&shaders.0));
    pipeline_state_descriptor.set_fragment_function(Some(&shaders.1));
    let attachment = pipeline_state_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap();
    attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

    device
        .new_render_pipeline_state(&pipeline_state_descriptor)
        .unwrap()
}

fn load_shaders(device: &Device) -> (Function, Function) {
    // TODO: include bytes
    let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("shaders/build/Release/shaders.metallib");

    let library = device.new_library_with_file(library_path).unwrap();
    let vert = library.get_function("full_screen_triangle", None).unwrap();
    let frag = library.get_function("trace_pixel", None).unwrap();

    (vert, frag)
}

fn init_view(app: &AppState, layer: &MetalLayer) {
    unsafe {
        let view = app.window.ns_view() as cocoa_id;
        view.setWantsLayer(YES);
        view.setLayer(mem::transmute(layer.as_ref()));
    }
}

fn init_layer(device: &Device, app: &AppState) -> MetalLayer {
    let layer = MetalLayer::new();
    layer.set_device(&device);
    layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    layer.set_presents_with_transaction(false);
    let draw_size = app.window.inner_size();
    layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));
    layer
}

fn init_buffer<T: Sized>(device: &Device, data: &[T]) -> Buffer {
    device.new_buffer_with_data(
        data.as_ptr() as *const _,
        (data.len() * mem::size_of::<T>()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
    )
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
