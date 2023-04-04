use spirv_std::glam::Vec4;

pub mod base;
pub mod render;

pub fn color_u32_from_vec4(v: Vec4) -> u32 {
    let convert = |f: f32| -> u32 { (f.clamp(0.0, 1.0) * 255.0).round() as u32 };

    convert(srgb_oetf(v.z))
        | convert(srgb_oetf(v.y)) << 8
        | convert(srgb_oetf(v.x)) << 16
        | convert(1.0) << 24
}

fn srgb_oetf(x: f32) -> f32 {
    if x <= 0.0031308 {
        x * 12.92
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}
