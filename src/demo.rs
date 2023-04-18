use std::f32::consts::PI;

use crate::shader_types::{Camera, PointLight, Shape, ShapeType, World};

use glam::{vec3, vec4, Mat4, Vec3A, vec3a};

pub fn chapter6() -> World {
    let mut world = base_world();
    world.camera.set_transform(Mat4::look_at_rh(
        vec3(0.0, 1.5, -5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
    ));
    let mut sphere = ShapeType::Sphere.create();
    sphere.material.colour = vec3a(1.0, 0.2, 1.0);
    world.add_shape(sphere);

    world
}

pub fn chapter11() -> World {
    let mut world = base_world();

    // less colour for less bright cause add
    let mut big = glass_sphere();
    big.set_transform(Mat4::from_translation(vec3(-0.5, 1.0, 0.5)));
    big.material.diffuse = 0.2;
    big.material.ambient = 0.01;
    world.add_shape(big);

    let mut small = glass_sphere();
    small.set_transform(Mat4::from_translation(vec3(-0.5, 1.0, 0.5)) * Mat4::from_scale(vec3(0.5, 0.5, 0.5)));
    small.material.diffuse = 0.2;
    small.material.ambient = 0.01;
    world.add_shape(small);

    let mut back = ShapeType::Sphere.create();
    back.material.colour = vec3a(0.9, 0.1, 0.1);
    back.set_transform(Mat4::from_translation(vec3(-0.5, 1.0, 1.5)) * Mat4::from_scale(vec3(0.5, 0.5, 0.5)));
    world.add_shape(back);

    let mut floor = ShapeType::Sphere.create();
    floor.shape = ShapeType::Plane;
    floor.material.colour.x = 0.8;
    floor.material.specular = 0.0;
    floor.material.reflective = 0.5;
    world.add_shape(floor);

    world
}

pub fn chapter9() -> World {
    let mut world = base_world();
    add_three_spheres(&mut world);

    let mut floor = ShapeType::Plane.create();
    floor.material.specular = 0.0;
    floor.material.reflective = 0.5;
    world.add_shape(floor);

    world
}

pub fn chapter7() -> World {
    let mut world = base_world();
    add_three_spheres(&mut world);

    let mut floor = ShapeType::Sphere.create();
    floor.set_transform(Mat4::from_scale(vec3(10.0, 0.01, 10.0)));
    floor.material.colour = Vec3A::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = ShapeType::Sphere.create();
    left_wall.set_transform(
        Mat4::from_translation(vec3(0.0, 0.0, 5.0))
            * Mat4::from_rotation_y(-PI / 4.0)
            * Mat4::from_rotation_x(PI / 2.0)
            * Mat4::from_scale(vec3(10.0, 0.01, 10.0)),
    );
    left_wall.material = floor.material;

    let mut right_wall = ShapeType::Sphere.create();
    right_wall.set_transform(
        Mat4::from_translation(vec3(0.0, 0.0, 5.0))
            * Mat4::from_rotation_y(PI / 4.0)
            * Mat4::from_rotation_x(PI / 2.0)
            * Mat4::from_scale(vec3(10.0, 0.01, 10.0)),
    );
    right_wall.material = floor.material;

    world.add_shape(floor);
    world.add_shape(left_wall);
    world.add_shape(right_wall);

    world
}

// TODO: import the yaml scene descriptions
//       https://forum.raytracerchallenge.com/board/4/gallery?q=scene+description
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

fn add_three_spheres(world: &mut World){
    let mut middle = ShapeType::Sphere.create();
    middle.set_transform(Mat4::from_translation(vec3(-0.5, 1.0, 0.5)));
    middle.material.colour = Vec3A::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.reflective = 0.15;
    world.add_shape(middle);

    let mut right = ShapeType::Sphere.create();
    right.set_transform(
        Mat4::from_translation(vec3(1.5, 0.5, -0.5)) * Mat4::from_scale(vec3(0.5, 0.5, 0.5)),
    );
    right.material.colour = Vec3A::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.reflective = 0.15;
    world.add_shape(right);

    let mut left = ShapeType::Sphere.create();
    left.set_transform(
        Mat4::from_translation(vec3(-1.5, 0.33, -0.75)) * Mat4::from_scale(vec3(0.33, 0.33, 0.33)),
    );
    left.material.colour = Vec3A::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    left.material.reflective = 0.15;
    world.add_shape(left);
}

fn glass_sphere() -> Shape {
    let mut s = ShapeType::Sphere.create();
    s.material.transparency = 0.5;
    s.material.refractive_index = 1.5;
    s
}
