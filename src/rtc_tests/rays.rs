use glam::{Mat4, vec3, Vec4};
use crate::shader_types::Ray;
use crate::rtc_tests::{point, vector};

// these don't really test anything interesting but seem like a good foundation to confirm that the ffi works.

#[test]
fn computing_a_point_from_a_distance(){
    let ray = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
    unsafe {
        assert_eq!(ray.position(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), point(4.5, 3.0, 4.0));
    }
}

#[test]
fn translating_a_ray(){
    transform_ray(Mat4::from_translation(vec3(3.0, 4.0, 5.0)),  point(4.0, 6.0, 8.0), vector(0.0, 1.0, 0.0));
}

#[test]
fn scaling_a_ray(){
    transform_ray(Mat4::from_scale(vec3(2.0, 3.0, 4.0)), point(2.0, 6.0, 12.0), vector(0.0, 3.0, 0.0));
}

fn transform_ray(m: Mat4, origin: Vec4, direction: Vec4) {
    let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let r2 = unsafe { r.transform(m) };
    assert_eq!(r2.origin, origin);
    assert_eq!(r2.direction, direction);
}