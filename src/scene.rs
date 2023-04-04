use std::mem::size_of;

use shaders::{
    material::Material,
    shapes::{Shape, ShapeType},
};
use spirv_std::glam::{vec2, vec3, vec4, Mat4, Vec3, Vec3A};

pub fn create_shapes() -> Vec<Shape> {
    vec![
        Shape {
            transform: Mat4::IDENTITY,
            shape: ShapeType::Sphere,
            id: 0,
            material: Material {
                colour: Vec3A::new(1.0, 0.2, 1.0),
                ..Default::default()
            },
        },
        Shape {
            transform: Mat4::from_translation(vec3(2.0, 0.0, 0.0)),
            shape: ShapeType::Sphere,
            id: 1,
            material: Material {
                colour: Vec3A::new(0.0, 0.7, 1.0),
                ..Default::default()
            },
        },
    ]
}
