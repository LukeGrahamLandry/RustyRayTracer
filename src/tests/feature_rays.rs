use glam::{Vec4, vec4};
use crate::shader_types::Ray;

#[test]
fn computing_a_point_from_a_distance(){
    let ray = Ray {
        origin: vec4(2.0, 3.0, 4.0, 1.0),
        direction: vec4(1.0, 0.0, 0.0, 0.0)
    };
    unsafe {
        let r = ray.position(0.0);
        assert_eq!(r,  vec4(2.0, 3.0, 4.0, 1.0));
        assert_eq!(ray.position(1.0),  vec4(3.0, 3.0, 4.0, 1.0));
        assert_eq!(ray.position(-1.0),  vec4(1.0, 3.0, 4.0, 1.0));
        assert_eq!(ray.position(2.5),  vec4(4.5, 3.0, 4.0, 1.0));
    }
}