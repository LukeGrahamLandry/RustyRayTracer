use shaders::{
    material::Material,
    shapes::{Shape, ShapeType},
};
use spirv_std::glam::{vec3, Mat4};

pub fn create_shapes() -> Vec<Shape> {
    vec![Shape {
        transform: Mat4::IDENTITY,
        shape: ShapeType::Sphere,
        id: 0,
        material: Material {
            colour: vec3(1.0, 0.2, 1.0),
            ..Default::default()
        },
    }]
}
