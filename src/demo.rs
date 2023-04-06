use std::f32::consts::PI;

use shaders::shapes::ShapeType;
use shaders::{camera::Camera, material::PointLight, shapes::Shape};
use spirv_std::glam::{vec3, vec4, Mat4, Vec3A};

use crate::scene::World;

pub fn chapter7() -> World {
    spheres_world(false)
}

pub fn chapter9() -> World {
    spheres_world(true)
}

// TODO: import the yaml scene descriptions
//       https://forum.raytracerchallenge.com/board/4/gallery?q=scene+description
pub fn spheres_world(use_plane: bool) -> World {
    let mut middle = Shape::default();
    middle.set_transform(Mat4::from_translation(vec3(-0.5, 1.0, 0.5)));
    middle.material.colour = Vec3A::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.reflective = 0.15;

    let mut right = Shape::default();
    right.set_transform(
        Mat4::from_translation(vec3(1.5, 0.5, -0.5)) * Mat4::from_scale(vec3(0.5, 0.5, 0.5)),
    );
    right.material.colour = Vec3A::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.reflective = 0.15;

    let mut left = Shape::default();
    left.set_transform(
        Mat4::from_translation(vec3(-1.5, 0.33, -0.75)) * Mat4::from_scale(vec3(0.33, 0.33, 0.33)),
    );
    left.material.colour = Vec3A::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    left.material.reflective = 0.15;

    let mut shapes = vec![middle, right, left];

    if use_plane {
        let mut floor = Shape::default();
        floor.shape = ShapeType::Plane;

        shapes.push(floor);
    } else {
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

        shapes.append(&mut vec![floor, left_wall, right_wall])
    }

    let mut camera = Camera::new(1280 / 2, 720 / 2, PI / 3.0);
    camera.set_transform(Mat4::look_at_rh(
        vec3(0.0, 1.5, -5.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
    ));

    let mut world = World {
        shapes,
        lights: vec![PointLight {
            position: vec4(-10.0, 10.0, -10.0, 1.0),
            intensity: Vec3A::new(1.0, 1.0, 1.0),
        }],
        camera,
    };

    world.assert_sanity();
    world
}