use std::f32::consts::PI;

use crate::shader_types::{Camera, PointLight, Shape, ShapeType, World};

use glam::{vec3, vec4, Mat4, Vec3A};

pub fn chapter9() -> World {
    let mut world = base_world();

    let mut floor = Shape::default();
    floor.shape = ShapeType::Plane;
    floor.material.specular = 0.0;
    floor.material.reflective = 0.5;
    world.add_shape(floor);

    world
}

pub fn chapter7() -> World {
    let mut world = base_world();

    let mut floor = Shape::default();
    floor.set_transform(Mat4::from_scale(vec3(10.0, 0.01, 10.0)));
    floor.material.colour = Vec3A::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Shape::default();
    left_wall.set_transform(
        Mat4::from_translation(vec3(0.0, 0.0, 5.0))
            * Mat4::from_rotation_y(-PI / 4.0)
            * Mat4::from_rotation_x(PI / 2.0)
            * Mat4::from_scale(vec3(10.0, 0.01, 10.0)),
    );
    left_wall.material = floor.material.clone();

    let mut right_wall = Shape::default();
    right_wall.set_transform(
        Mat4::from_translation(vec3(0.0, 0.0, 5.0))
            * Mat4::from_rotation_y(PI / 4.0)
            * Mat4::from_rotation_x(PI / 2.0)
            * Mat4::from_scale(vec3(10.0, 0.01, 10.0)),
    );
    right_wall.material = floor.material.clone();

    world.add_shape(floor);
    world.add_shape(left_wall);
    world.add_shape(right_wall);

    world
}

// TODO: import the yaml scene descriptions
//       https://forum.raytracerchallenge.com/board/4/gallery?q=scene+description
fn base_world() -> World {
    let mut world = World::default();
    world.camera = Camera::new(1000, 500, PI / 3.0);
    world.camera.set_transform(Mat4::look_at_rh(
        vec3(0.0, 1.5, -5.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
    ));
    world.add_light(PointLight {
        position: vec4(-10.0, 10.0, -10.0, 1.0),
        intensity: Vec3A::new(1.0, 1.0, 1.0),
    });
    add_three_spheres(&mut world);

    world
}

fn add_three_spheres(world: &mut World){
    let mut middle = Shape::default();
    middle.set_transform(Mat4::from_translation(vec3(-0.5, 1.0, 0.5)));
    middle.material.colour = Vec3A::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.reflective = 0.15;
    world.add_shape(middle);

    let mut right = Shape::default();
    right.set_transform(
        Mat4::from_translation(vec3(1.5, 0.5, -0.5)) * Mat4::from_scale(vec3(0.5, 0.5, 0.5)),
    );
    right.material.colour = Vec3A::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.reflective = 0.15;
    world.add_shape(right);

    let mut left = Shape::default();
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
    let mut s = Shape::default();
    s.material.transparency = 1.0;
    s.material.refractive_index = 1.5;
    s
}
