#![no_std]

pub mod ray;
pub mod shapes;
pub mod camera;
pub mod material;

use core::f32::consts::PI;
use spirv_std::spirv;
use spirv_std::glam::{Mat4, vec2, Vec2, Vec3, vec3, Vec3Swizzles, vec4, Vec4, Vec4Swizzles};
use crate::camera::Camera;
use crate::material::{Material, PointLight};
use crate::ray::Intersections;
use crate::shapes::{Shape, ShapeType};

pub struct ShaderConstants {
    pub time: f32
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] pixel_pos: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] shapes: &mut [Shape],
    out_colour: &mut Vec4
) {
    // TODO: put the camera in the constants so i just make it once, and dont need the window size here
    let mut camera = Camera::new(1280, 720, PI / 6.0);
    let pos = vec4(0.0, 10.0, 1.1, 1.0);
    let pos = Mat4::from_rotation_y(constants.time) * pos;
    camera.set_transform(Mat4::look_at_lh(pos.xyz(), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0)));

    let sphere = Shape {
        transform: Mat4::IDENTITY,
        shape: ShapeType::Sphere,
        id: 0,
        material: Material {
            colour: vec3(1.0, 0.2, 1.0),
            ..Default::default()
        },
    };

    let light = PointLight {
        position: vec4(-20.0, 10.0, 30.0, 1.0),
        intensity: vec3(1.0, 1.0, 1.0),
    };

    let ray = camera.ray_for_pixel(pixel_pos.x, pixel_pos.y);
    let mut hits = Intersections::default();
    sphere.intersect(&ray, &mut hits);

    if hits.has_hit() {
        let hit_pos = ray.position(hits.get_hit().t);
        let colour = sphere.material.lighting(light, hit_pos, -ray.direction, sphere.normal_at(hit_pos), false);
        *out_colour = colour.xyzz();
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
