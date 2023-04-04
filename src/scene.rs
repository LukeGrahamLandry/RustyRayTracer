use std::mem::size_of;

use shaders::{
    material::Material,
    shapes::{Shape, ShapeType},
};
use spirv_std::glam::{vec2, vec3, vec4, Mat4, Vec3, Vec3A};
use shaders::material::PointLight;

pub struct World {
    pub shapes: Vec<Shape>,
    pub lights: Vec<PointLight>
}

impl World {
    /// Checks that the ids are correct indexes into the arrays.
    pub fn assert_sanity(&self) {
        for (i, shape) in self.shapes.iter().enumerate() {
            debug_assert_eq!(i as u32, shape.id);
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let shapes = vec![
            Shape {
                transform_inverse: Mat4::IDENTITY,
                shape: ShapeType::Sphere,
                id: 0,
                material: Material {
                    colour: Vec3A::new(1.0, 0.2, 1.0),
                    ..Default::default()
                },
            },
            Shape {
                transform_inverse: Mat4::from_translation(vec3(2.0, 0.0, 0.0)).inverse(),
                shape: ShapeType::Sphere,
                id: 1,
                material: Material {
                    colour: Vec3A::new(0.0, 0.7, 1.0),
                    ..Default::default()
                },
            },
        ];

        let lights = vec![
            PointLight {
                position: vec4(-20.0, 10.0, 30.0, 1.0),
                intensity: Vec3A::new(1.0, 1.0, 1.0),
            }
        ];

        let world = World {
            shapes,
            lights
        };

        world.assert_sanity();
        world
    }
}
