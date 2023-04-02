#![no_std]

use spirv_std::spirv;
use spirv_std::glam::{vec2, Vec2, vec4, Vec4};

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(position)] pixel_pos: Vec4,
    out_colour: &mut Vec4
) {
    *out_colour = vec4(pixel_pos.x / 500.0, pixel_pos.y / 500.0, 0.0, 1.0);
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
