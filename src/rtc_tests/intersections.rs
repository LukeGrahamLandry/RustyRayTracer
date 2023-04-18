use crate::rtc_tests::{point, vector};
use crate::shader_types::{Intersection, Intersections, Ray, ShapeType, World};

#[test]
fn precomputing_the_state_of_an_intersection() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut w = World::default();
    w.add_shape(ShapeType::Sphere.create());

    let i = Intersection::new(4.0, 0);
    let comps = unsafe { w.view().prepare_comps(&i, &r, &Intersections::default()) };

    assert_eq!(comps.point, point(0.0, 0.0, -1.0));
    assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
}
