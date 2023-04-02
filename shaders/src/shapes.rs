use spirv_std::glam::{Mat4, Vec4, vec4};
use crate::ray::{Intersection, Intersections, Ray};
use spirv_std::num_traits::Float;

pub enum ShapeType {
    Sphere
}

pub struct Shape {
    pub transform: Mat4,
    pub shape: ShapeType,
    pub id: usize
}

impl Shape {
    pub fn intersect(&self, world_ray: &Ray, hits: &mut Intersections) {
        let object_ray = world_ray.transform(self.transform.inverse());
        self.local_intersect(object_ray, hits);
    }

    pub fn local_intersect(&self, object_ray: Ray, hits: &mut Intersections) {
        match self.shape {
            ShapeType::Sphere => {
                self.local_intersect_sphere(object_ray, hits);
            }
        }
    }

    fn local_intersect_sphere(&self, ray: Ray, hits: &mut Intersections) {
        let sphere_to_ray = ray.origin - vec4(0.0, 0.0, 0.0, 1.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            hits.add(Intersection {
                t: t1,
                obj: self.id,
            });
            hits.add(Intersection {
                t: t2,
                obj: self.id,
            });
        }
    }
}
