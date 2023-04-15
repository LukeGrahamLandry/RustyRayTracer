mod feature_rays;

use std::slice;
use std::mem::{align_of, size_of};
use glam::{Mat4, vec3, vec3a, vec4};
use crate::shader_types::{Camera, Material, PointLight, ShaderInputs, Shape, ShapeType, World, WorldView};

fn default_world() -> World {
    let mut world = World::default();
    let mut sphere = Shape::default();
    sphere.material.colour = vec3a(0.8, 1.0, 0.6);
    sphere.material.diffuse = 0.7;
    sphere.material.specular = 0.2;
    world.add_shape(sphere);
    let mut sphere = Shape::default();
    sphere.set_transform(Mat4::from_scale(vec3(0.5, 0.5, 0.5)));
    world.add_shape(sphere);
    world.add_light(PointLight {
        position: vec4(-10.0, 10.0, -10.0, 0.0),
        intensity: vec3a(1.0, 1.0, 1.0),
    });

    world
}
