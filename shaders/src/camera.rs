use spirv_std::glam::{Mat4, Vec4, vec4};
use spirv_std::num_traits::Float;

use crate::ray::Ray;

pub struct Camera {
    transform: Mat4,
    hsize: usize,
    vsize: usize,
    field_of_view: f32,  // radians!
    pixel_size: f32,
    half_width: f32,
    half_height: f32
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect_ratio = (hsize as f32) / (vsize as f32);
        let half_width = if aspect_ratio >= 1.0 { half_view } else { half_view * aspect_ratio };
        let half_height = if aspect_ratio >= 1.0 { half_view / aspect_ratio } else { half_view };

        Camera {
            transform: Mat4::IDENTITY,
            hsize,
            vsize,
            half_width,
            half_height,
            field_of_view,
            pixel_size: (half_width * 2.0) / (hsize as f32)
        }
    }

    pub fn set_transform(&mut self, mat: Mat4) {
        self.transform = mat;
    }

    pub fn ray_for_pixel(&self, x: f32, y: f32) -> Ray {
        // We want the middle of the pixel.
        // Adjusted from canvas space to world space units.
        // Since the camera is at (0, 0), translate. This flips it so high y becomes negative.
        // But canvas units are kinda flipped too, so it cancels out? And canvas looks at -x so x flip works too.
        let object_x = self.half_width - ((x + 0.5) * self.pixel_size);
        let object_y = self.half_height - ((y + 0.5) * self.pixel_size);

        // Position of the pixel in the camera's object space.
        let pixel_object_point = vec4(object_x, object_y, /* - */ 1.0, 1.0);

        // Transform to world space.
        let pixel_world_point = self.transform.inverse() * pixel_object_point;
        let origin = self.transform.inverse() * vec4(0.0, 0.0, 0.0, 1.0);
        let direction = pixel_world_point - origin;
        Ray { origin, direction }
    }
}
