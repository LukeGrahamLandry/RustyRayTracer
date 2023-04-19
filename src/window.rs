use std::{env, fs};
use std::time::Instant;

use crate::controller::CameraController;
use crate::demo::*;
use crate::shader_types::World;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, ElementState, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};
use crate::scene::{load_scene, SCENE_FILES};

pub trait RenderStrategy: Sized + 'static {
    fn new(app: &AppState) -> Self;
    fn render(&mut self, app: &AppState);
    fn resized(&mut self, size: LogicalSize<u32>);
    fn world_changed(&mut self, app: &AppState);

    fn run() {
        println!("Start.");
        let (app, event_loop) = AppState::new();
        let renderer = Self::new(&app);
        app.run(renderer, event_loop);
    }
}

pub struct AppState {
    pub window: Window,
    pub world: World,
    timer: FrameTimer,
    controller: CameraController,
}

/// All the logic for creating a window and handling events that can be shared between gpu and cpu renderers.
impl AppState {
    pub fn new() -> (AppState, EventLoop<()>) {
        println!(
            "Use the number keys to switch between included scenes. The window can be resized."
        );
        let world = initial_world();
        let event_loop = winit::event_loop::EventLoop::new();
        let size = LogicalSize::new(world.camera.size().0, world.camera.size().1);

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(size)
            .with_title("Rusty Raytracer")
            .build(&event_loop)
            .unwrap();

        (
            AppState {
                window,
                world,
                timer: FrameTimer::new(),
                controller: CameraController::default(),
            },
            event_loop,
        )
    }

    pub fn run<T: RenderStrategy>(mut self, mut renderer: T, event_loop: EventLoop<()>) {
        println!("Starting event loop.");
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        self.controller.keyboard_event(input);
                        if input.state == ElementState::Pressed {
                            match input.virtual_keycode {
                                Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                                key => {
                                    if let Some(w) = preset_world(key) {
                                        println!("Switch scene."); // why tf am i at 10 levels of indentation
                                        let size = LogicalSize::new(w.camera.hsize, w.camera.vsize);
                                        self.window.set_inner_size(size);
                                        self.world = w;
                                        self.resize_camera();
                                        renderer.world_changed(&self);
                                    }
                                }
                            }
                        }
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(_) => {
                        renderer.resized(self.resize_camera());
                    }
                    _ => (),
                },
                // Nested destructuring, my beloved
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta, .. },
                    ..
                } => {
                    self.controller.mouse_moved(delta);
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    self.controller.update(
                        &mut self.world.camera,
                        self.timer.last.elapsed().as_secs_f32(),
                    );
                    renderer.render(&self);
                    self.timer.update();
                }
                _ => {}
            }
        });
    }

    fn resize_camera(&mut self) -> LogicalSize<u32> {
        let size: LogicalSize<u32> =
            LogicalSize::from_physical(self.window.inner_size(), self.window.scale_factor());
        self.world
            .camera
            .resize(size.width as usize, size.height as usize);
        println!("Resolution: {}x{}", size.width, size.height);
        size
    }
}

fn initial_world() -> World {
    let args: Vec<String> = env::args().collect();
    for name in args {
        if let Ok(data) = fs::read_to_string(&name) {
            if let Ok(world) = load_scene(&data) {
                return world;
            }
        }
    }

    load_scene(SCENE_FILES[0]).unwrap()
}
fn preset_world(key: Option<VirtualKeyCode>) -> Option<World> {
    if let Some(k) = key{
        let index = ((k as u32) - (VirtualKeyCode::Key1 as u32)) as usize;
        // Can't have more than 10 presets because the next key in the enum is 'A' which I want to use for movement.
        if index < SCENE_FILES.len() && index <= 10 {
            return Some(load_scene(SCENE_FILES[index]).unwrap());
        }
    }

    None
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

    pub fn update(&mut self) {
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
        println!(
            "{} seconds; {} frames; {} fps; {} ms per frame;",
            seconds,
            self.frame_count,
            fps.round(),
            frame_time_ms
        );
        self.micro_seconds = 0;
        self.frame_count = 0;
    }
}
