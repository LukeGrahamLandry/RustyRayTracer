use crate::shader_types::Camera;
use glam::{vec3, Mat4, Vec2, Vec3};
use winit::event::ElementState;
use winit::event::{KeyboardInput, VirtualKeyCode};

#[derive(Default)]
pub struct CameraController {
    left: bool,
    right: bool,
    forward: bool,
    back: bool,
    up: bool,
    down: bool,
    rotation: Vec2,
}

const MOVE_SPEED: f32 = 150.0;

impl CameraController {
    pub fn keyboard_event(&mut self, event: KeyboardInput) {
        // This could be a hash map but this is a heap-less home.
        match event.virtual_keycode {
            Some(VirtualKeyCode::W) => {
                self.forward = event.state == ElementState::Pressed;
            }
            Some(VirtualKeyCode::S) => {
                self.back = event.state == ElementState::Pressed;
            }
            Some(VirtualKeyCode::A) => {
                self.left = event.state == ElementState::Pressed;
            }
            Some(VirtualKeyCode::D) => {
                self.right = event.state == ElementState::Pressed;
            }
            Some(VirtualKeyCode::Space) => {
                self.up = event.state == ElementState::Pressed;
            }
            Some(VirtualKeyCode::LShift) => {
                self.down = event.state == ElementState::Pressed;
            }
            _ => {}
        }
    }

    pub fn mouse_moved(&mut self, delta_mouse: (f64, f64)) {
        self.rotation.x += delta_mouse.0 as f32;
        self.rotation.y += delta_mouse.1 as f32;
    }

    pub fn update(&mut self, camera: &mut Camera, dt: f32) {
        let last_matrix = camera.get_transform();
        let (scale, rotation, mut translation) = last_matrix.to_scale_rotation_translation();
        translation += self.direction() * dt * MOVE_SPEED;
        camera.set_transform(Mat4::from_scale_rotation_translation(
            scale,
            rotation,
            translation,
        ));
        self.rotation = Vec2::ZERO;
    }

    fn direction(&self) -> Vec3 {
        vec3(
            numberify(self.right, self.left),
            numberify(self.down, self.up),
            numberify(self.forward, self.back),
        )
    }
}

fn numberify(positive: bool, negative: bool) -> f32 {
    if negative {
        -1.0
    } else if positive {
        1.0
    } else {
        0.0
    }
}
