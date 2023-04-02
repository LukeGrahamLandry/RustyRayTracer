#![no_std]

mod ray;
mod shapes;
mod camera;

use core::f32::consts::PI;
use spirv_std::spirv;
use spirv_std::glam::{Mat4, vec2, Vec2, Vec3, vec3, vec4, Vec4};
use crate::camera::Camera;
use crate::ray::Intersections;
use crate::shapes::{Shape, ShapeType};

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(position)] pixel_pos: Vec4,
    out_colour: &mut Vec4
) {
    let mut camera = Camera::new(100, 50, PI / 3.0);
    camera.set_transform(Mat4::look_at_lh(vec3(10.0, 10.0, 10.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0)));

    let sphere = Shape {
        transform: Mat4::IDENTITY,
        shape: ShapeType::Sphere,
        id: 0,
    };

    let ray = camera.ray_for_pixel(pixel_pos.x, pixel_pos.y);
    let mut hits = Intersections::default();
    sphere.intersect(&ray, &mut hits);

    if hits.has_hit() {
        *out_colour = vec4(1.0, 0.0, 0.0, 1.0);
    } else {
        *out_colour = vec4(0.0, 0.0, 0.0, 1.0);
    }
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
