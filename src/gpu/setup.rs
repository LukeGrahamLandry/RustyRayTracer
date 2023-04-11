extern crate objc;

use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;

use metal::*;
use objc::{rc::autoreleasepool, runtime::YES};
use std::mem;
use std::time::Instant;
use winit::platform::macos::WindowExtMacOS;

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;
use winit::window::Window;
use crate::scene::World;
use crate::timer::FrameTimer;
use crate::types::{PointLight, Shape};

pub struct AppState {
    pub(crate) layer: MetalLayer,
    window: Window,
    pub(crate) command_queue: CommandQueue,
    pub(crate) pipeline_state: RenderPipelineState,
    timer: FrameTimer,
    pub(crate) start: Instant,
    pub(crate) world: World,
    pub shapes_buffer: Buffer,
    pub lights_buffer: Buffer,
}

impl AppState {
    pub fn new(world: World) -> (AppState, EventLoop<()>) {
        let event_loop = winit::event_loop::EventLoop::new();
        let size = winit::dpi::LogicalSize::new(world.camera.size().0, world.camera.size().1);

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(size)
            .with_title("Metal Window Example".to_string())
            .build(&event_loop)
            .unwrap();

        let device = Device::system_default().expect("no device found");

        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        unsafe {
            let view = window.ns_view() as cocoa_id;
            view.setWantsLayer(YES);
            view.setLayer(mem::transmute(layer.as_ref()));
        }

        let draw_size = window.inner_size();
        layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));

        let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/Build/Products/Debug/shaders.metallib");
        // .join("src/gpu/shaders/shaders.metallib");
        // xcrun -sdk macosx metal -c src/gpu/shaders/shaders.metal -o src/gpu/shaders/shaders.air && xcrun -sdk macosx metallib src/gpu/shaders/shaders.air -o src/gpu/shaders/shaders.metallib

        let library = device.new_library_with_file(library_path).unwrap();
        let vert = library.get_function("full_screen_triangle", None).unwrap();
        let frag = library.get_function("raytracer_fragment", None).unwrap();

        let pipeline_state_descriptor = RenderPipelineDescriptor::new();
        pipeline_state_descriptor.set_vertex_function(Some(&vert));
        pipeline_state_descriptor.set_fragment_function(Some(&frag));
        let attachment = pipeline_state_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

        let pipeline_state = device
            .new_render_pipeline_state(&pipeline_state_descriptor)
            .unwrap();

        let command_queue = device.new_command_queue();

        let shapes_buffer = device.new_buffer_with_data(
            world.shapes.as_ptr() as *const _,
            (world.shapes.len() * mem::size_of::<Shape>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        );

        let lights_buffer = device.new_buffer_with_data(
            world.lights.as_ptr() as *const _,
            (world.lights.len() * mem::size_of::<PointLight>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        );

        (AppState {
            layer,
            window,
            command_queue,
            pipeline_state,
            timer: FrameTimer::new(),
            world,
            start: Instant::now(),
            shapes_buffer,
            lights_buffer
        }, event_loop)
    }

    pub fn tick(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        autoreleasepool(|| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                        Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        self.layer.set_drawable_size(CGSize::new(size.width as f64, size.height as f64));
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    self.render();
                    self.timer.update();
                }
                _ => {}
            }
        });
    }
}
