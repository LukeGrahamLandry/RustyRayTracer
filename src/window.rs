use std::time::Instant;

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{ElementState, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::Window;
use crate::demo::*;
use crate::shader_types::{ShaderInputs, World};

pub trait RenderStrategy: Sized + 'static {
    fn new(app: &AppState) -> Self;
    fn render(&mut self, app: &AppState);
    fn resized(&mut self, size: LogicalSize<u32>);
    fn world_changed(&mut self, app: &AppState);

    fn run() {
        let (app, event_loop) = AppState::new();
        let renderer = Self::new(&app);
        app.run(renderer, event_loop);
    }
}

pub struct AppState {
    pub window: Window,
    pub world: World,
    timer: FrameTimer,
    start: Instant,
}

/// All the logic for creating a window and handling events that can be shared between gpu and cpu renderers.
impl AppState {
    pub fn new() -> (AppState, EventLoop<()>) {
        let world = chapter7();
        let event_loop = winit::event_loop::EventLoop::new();
        let size = winit::dpi::LogicalSize::new(world.camera.size().0, world.camera.size().1);

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(size)
            .with_title("Rusty Raytracer")
            .build(&event_loop)
            .unwrap();

        (AppState {
            window,
            world,
            timer: FrameTimer::new(),
            start: Instant::now()
        }, event_loop)
    }

    pub fn shader_inputs(&self) -> ShaderInputs {
        ShaderInputs {
            time: (Instant::now() - self.start).as_secs_f32(),
            camera: self.world.camera,
            shape_count: self.world.get_shapes().len() as u32,
            light_count: self.world.get_lights().len() as u32
        }
    }

    pub fn run<T: RenderStrategy>(mut self, mut renderer: T, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input, .. } => if input.state == ElementState::Pressed {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                            Some(VirtualKeyCode::Key1) => {
                                self.world = chapter7();
                                self.resize_camera();
                                renderer.world_changed(&mut self);
                            },
                            Some(VirtualKeyCode::Key2) => {
                                self.world = chapter9();
                                self.resize_camera();
                                renderer.world_changed(&mut self);
                            }
                            _ => {}
                        }
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(_) => {
                        renderer.resized(self.resize_camera());
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    renderer.render(&mut self);
                    self.timer.update();
                }
                _ => {}
            }
        });
    }

    fn resize_camera(&mut self) -> LogicalSize<u32>{
        let size: LogicalSize<u32> = LogicalSize::from_physical(self.window.inner_size(), self.window.scale_factor());
        self.world.camera.resize(size.width as usize, size.height as usize);
        println!("Resolution: {}x{}", size.width, size.height);
        size
    }
}

pub struct FrameTimer {
    pub frame_count: i32,
    pub micro_seconds: u128,
    pub last: Instant,
}

impl FrameTimer {
    pub fn new() -> Self {
        FrameTimer {
            frame_count: 0,
            micro_seconds: 0,
            last: Instant::now(),
        }
    }

    pub fn update(&mut self){
        let now = Instant::now();
        self.micro_seconds += self.last.elapsed().as_micros();
        self.last = now;
        self.frame_count += 1;

        if self.micro_seconds > 2000000 {
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        let seconds = self.micro_seconds as f64 / 1000000.0;
        let frame_time_ms = (self.micro_seconds as f64 / self.frame_count as f64).round() / 1000.0;
        let fps = self.frame_count as f64 / seconds;
        println!("{} seconds; {} frames; {} fps; {} ms per frame;", seconds, self.frame_count, fps.round(), frame_time_ms);
        self.micro_seconds = 0;
        self.frame_count = 0;
    }
}
