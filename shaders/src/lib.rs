#![no_std]

pub mod camera;
pub mod material;
pub mod ray;
pub mod shapes;
pub mod world;

use crate::camera::Camera;
use crate::material::PointLight;

use crate::shapes::Shape;
use crate::world::WorldView;

use spirv_std::glam::{vec2, Vec2, Vec3Swizzles, Vec4, Vec4Swizzles};
use spirv_std::spirv;

pub struct ShaderInputs {
    pub time: f32,
    pub camera: Camera,
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] pixel_pos: Vec4,
    #[spirv(push_constant)] inputs: &ShaderInputs,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] shapes: &[Shape],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] lights: &[PointLight],
    out_colour: &mut Vec4,
) {
    let world = WorldView { shapes, lights };
    let ray = inputs.camera.ray_for_pixel(pixel_pos.x, pixel_pos.y);
    *out_colour = world.color_at(ray).xyzz();
}

// Big triangle that covers the screen so the fragment shader runs for every pixel.
// https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    let uv = vec2(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *out_pos = pos.extend(0.0).extend(1.0);
}
