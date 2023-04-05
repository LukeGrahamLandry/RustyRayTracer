use std::mem::size_of;

use shaders::camera::Camera;
use shaders::material::PointLight;
use shaders::{
    material::Material,
    shapes::{Shape, ShapeType},
};
use spirv_std::glam::{vec2, vec3, vec4, Mat4, Vec3, Vec3A};

use crate::demo::chapter7;

pub struct World {
    pub shapes: Vec<Shape>,
    pub lights: Vec<PointLight>,
    pub camera: Camera,
}

impl World {
    /// Checks that the ids are correct indexes into the arrays.
    pub fn assert_sanity(&mut self) {
        for (i, shape) in self.shapes.iter_mut().enumerate() {
            shape.index = i as u32;
        }
    }
}
