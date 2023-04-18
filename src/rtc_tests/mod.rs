mod intersections;
mod rays;

use crate::bindings::Ray;
use crate::shader_types::{Intersection, Intersections, PointLight, ShapeType, World, MAX_HITS};
use glam::{vec3, vec3a, vec4, Mat4, Vec4};

fn default_test_world() -> World {
    let mut world = World::default();
    let mut sphere = ShapeType::Sphere.create();
    sphere.material.colour = vec3a(0.8, 1.0, 0.6);
    sphere.material.diffuse = 0.7;
    sphere.material.specular = 0.2;
    world.add_shape(sphere);
    let mut sphere = ShapeType::Sphere.create();
    sphere.set_transform(Mat4::from_scale(vec3(0.5, 0.5, 0.5)));
    world.add_shape(sphere);
    world.add_light(PointLight {
        position: vec4(-10.0, 10.0, -10.0, 0.0),
        intensity: vec3a(1.0, 1.0, 1.0),
    });

    world
}

fn vector(x: f32, y: f32, z: f32) -> Vec4 {
    vec4(x, y, z, 0.0)
}

fn point(x: f32, y: f32, z: f32) -> Vec4 {
    vec4(x, y, z, 1.0)
}

impl Default for Intersection {
    fn default() -> Self {
        Intersection::new(0.0, 0)
    }
}

impl Default for Intersections {
    fn default() -> Self {
        Intersections {
            count: 0,
            is_hit: false,
            hits: [Default::default(); MAX_HITS as usize],
        }
    }
}

impl Intersection {
    fn new(t: f32, obj: u32) -> Intersection {
        Intersection { t, obj }
    }
}

impl Ray {
    fn new(origin: Vec4, direction: Vec4) -> Ray {
        Ray { origin, direction }
    }
}
