use std::f32::consts::PI;

use crate::shader_types::{Camera, PatternType, PointLight, Shape, ShapeType, World};

use glam::{vec3, vec3a, vec4, Mat4, Vec3A};
use crate::bindings::Pattern;

fn base_world() -> World {
    let mut world = World {
        camera: Camera::new(1000, 500, PI / 3.0),
        ..Default::default()
    };
    world.camera.set_transform(Mat4::look_at_rh(
        vec3(0.0, 1.5, -5.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
    ));
    world.add_light(PointLight {
        position: vec4(-10.0, 10.0, -10.0, 1.0),
        intensity: Vec3A::new(1.0, 1.0, 1.0),
    });

    world
}

pub fn chapter10() -> World {
    let mut world = base_world();

    let mut middle = ShapeType::Sphere.create();
    middle.set_transform(Mat4::from_translation(vec3(-0.5, 1.0, 0.5)));
    middle.material.colour = Vec3A::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.reflective = 0.15;
    middle.material.pattern_index = world.add_pattern(Pattern {
        a: vec3a(0.5, 0.5, 0.5),
        b: vec3a(0.8, 0.2, 0.2),
        pattern: PatternType::Stripes,
        __bindgen_padding_0: 0,
        transform_inverse: Mat4::IDENTITY,
    });
    world.add_shape(middle);

    let mut left = ShapeType::Sphere.create();
    left.set_transform(
        Mat4::from_translation(vec3(-1.5, 0.33, -0.75)) * Mat4::from_scale(vec3(0.33, 0.33, 0.33)),
    );
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    left.material.reflective = 0.15;
    left.material.pattern_index = world.add_pattern(Pattern {
        a: vec3a(0.5, 0.5, 0.5),
        b: vec3a(0.8, 0.2, 0.2),
        pattern: PatternType::Stripes,
        __bindgen_padding_0: 0,
        transform_inverse: Mat4::IDENTITY,
    });
    world.add_shape(left);

    let mut right = ShapeType::Sphere.create();
    right.set_transform(
        Mat4::from_translation(vec3(1.5, 0.5, -0.5)) * Mat4::from_scale(vec3(0.5, 0.5, 0.5)),
    );
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.reflective = 0.15;
    right.material.pattern_index = world.add_pattern(Pattern {
        a: vec3a(0.0, 0.0, 1.0),
        b: vec3a(1.0, 0.0, 0.0),
        pattern: PatternType::Gradient,
        __bindgen_padding_0: 0,
        transform_inverse: Mat4::from_scale(vec3(0.5, 0.5, 0.5)) * Mat4::from_translation(vec3(1.0, 0.0, 0.0)),
    });
    world.add_shape(right);

    let mut another = ShapeType::Sphere.create();
    another.set_transform(
        Mat4::from_translation(vec3(2.0, 1.0, 4.0)),
    );
    another.material.diffuse = 0.7;
    another.material.specular = 0.3;
    another.material.reflective = 0.15;
    another.material.pattern_index = world.add_pattern(Pattern {
        a: vec3a(1.0, 1.0, 1.0),
        b: vec3a(0.0, 1.0, 0.0),
        pattern: PatternType::Checker,
        __bindgen_padding_0: 0,
        transform_inverse: Mat4::from_scale(vec3(4.0, 4.0, 4.0)),
    });
    world.add_shape(another);


    let mut floor = ShapeType::Plane.create();
    floor.material.specular = 0.0;
    floor.material.reflective = 0.5;
    floor.material.pattern_index = world.add_pattern(Pattern {
        a: vec3a(0.5, 0.5, 0.5),
        b: vec3a(0.8, 0.2, 0.2),
        pattern: PatternType::Ring,
        __bindgen_padding_0: 0,
        transform_inverse: Mat4::from_rotation_y(PI / 2.0),
    });
    world.add_shape(floor);

    world

}

fn glass_sphere() -> Shape {
    let mut s = ShapeType::Sphere.create();
    s.material.transparency = 1.0;
    s.material.refractive_index = 1.5;
    s
}
