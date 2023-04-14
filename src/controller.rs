use std::f32::consts::PI;
use glam::{DVec2, EulerRot, Mat4, Quat, Vec2, vec3, Vec3, vec4, Vec4, Vec4Swizzles};
use winit::event::{Event, KeyboardInput, VirtualKeyCode};
use winit::event::ElementState;
use crate::shader_types::Camera;

#[derive(Default)]
pub struct CameraController {
    left: bool,
    right: bool,
    forward: bool,
    back: bool,
    up: bool,
    down: bool,
    rotation: Vec2
}

impl CameraController {
    pub fn keyboard_event(&mut self, event: KeyboardInput) {
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

    pub fn update(&mut self, camera: &mut Camera, dt: f32){
        let MOVE_SPEED = 150.0;
        let ROT_SPEED = 0.05;
        let last_matrix = camera.get_transform();
        let (scale, rotation, mut translation) = last_matrix.to_scale_rotation_translation();
        let un_translate = Mat4::from_translation(translation).inverse();
        let (_, rotation, _) = (un_translate * last_matrix).to_scale_rotation_translation();
        translation += self.direction() * dt * MOVE_SPEED;
        // let rot = Quat::from_rotation_y(self.rotation.x * ROT_SPEED) * Quat::from_rotation_x(self.rotation.y * ROT_SPEED);
        // rotation *= rot;
        let (mut sideways, mut up, nothing) = rotation.to_euler(EulerRot::YXZ);
        sideways += self.rotation.x * ROT_SPEED;
        // up += self.rotation.y * ROT_SPEED;
        // up = up.clamp(0.0, PI);
        let rot = Quat::from_euler(EulerRot::YXZ, sideways, up, nothing);
        camera.set_transform(Mat4::from_translation(translation) * Mat4::from_quat(rot));
        self.rotation = Vec2::ZERO;
    }

    fn direction(&self) -> Vec3 {
        let x = if self.right {
            1.0
        } else if self.left {
            -1.0
        } else {
            0.0
        };
        let y = if self.up {
            -1.0
        } else if self.down {
            1.0
        } else {
            0.0
        };
        let z = if self.forward {
            1.0
        } else if self.back {
            -1.0
        } else {
            0.0
        };

        vec3(x, y, z)
    }
}