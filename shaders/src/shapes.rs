use crate::material::Material;
use crate::ray::{Intersection, Intersections, Ray};
use spirv_std::glam::{vec4, Mat4, Vec2, Vec3, Vec4};
use spirv_std::num_traits::Float;

// even if this only has one variant and could be zero sized, it MUST be repr(C) and waste space to make it work in storage buffers.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ShapeType {
    Sphere,
}

#[repr(C)]
pub struct Shape {
    pub transform_inverse: Mat4,
    pub shape: ShapeType,
    // usize is 8 bytes on the cpu but 4 bytes on the gpu so never ever ever use it in structs
    pub id: u32,
    pub material: Material,
}

impl Shape {
    pub fn set_transform(&mut self, mat: Mat4) {
        self.transform_inverse = mat.inverse();
    }

    pub fn intersect(&self, world_ray: &Ray, hits: &mut Intersections) {
        let object_ray = world_ray.transform(self.transform_inverse);
        self.local_intersect(object_ray, hits);
    }

    pub fn normal_at(&self, world_space_point: Vec4) -> Vec4 {
        let object_space_point = self.transform_inverse * world_space_point;
        let object_space_normal = self.local_normal_at(object_space_point);
        let mut world_space_normal = self.transform_inverse.transpose() * object_space_normal;
        world_space_normal.w = 0.0;
        world_space_normal.normalize()
    }

    pub fn local_intersect(&self, object_ray: Ray, hits: &mut Intersections) {
        match self.shape {
            ShapeType::Sphere => {
                self.local_intersect_sphere(object_ray, hits);
            }
        }
    }

    pub fn local_normal_at(&self, object_space_point: Vec4) -> Vec4 {
        match self.shape {
            ShapeType::Sphere => object_space_point - vec4(0.0, 0.0, 0.0, 1.0),
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
