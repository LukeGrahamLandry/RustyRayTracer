use crate::material::{reflect_vec, PointLight};
use crate::ray::{Comps, Intersection, Intersections, Ray};
use crate::shapes::Shape;
use spirv_std::arch::IndexUnchecked;
use spirv_std::glam::{Vec3A, Vec4};
use spirv_std::num_traits::Float;

pub struct WorldView<'a> {
    pub shapes: &'a [Shape],
    pub lights: &'a [PointLight],
}

pub const EPSILON: f32 = 0.01;
pub const REFLECTION_DEPTH: u32 = 5;

impl<'a> WorldView<'a> {
    // Be careful not to try to return Intersections by value from a function.
    // It breaks the msl code gen when it tries to copy the array.
    // It generates spvArrayCopyFromStackToStack1 and the generics get messed up somehow.
    // I wonder if passing --msl-force-native-arrays to spirv-cross would also fix it but idk how to do that.

    // The gpu really hates recursion so we gotta unwrap the reflection algorithim into something itterative.
    pub fn color_at(&self, mut ray: Ray) -> Vec3A {
        let mut colour = Vec3A::ZERO;
        let mut prev_reflectance = 1.0;
        for _ in 0..REFLECTION_DEPTH {
            let mut hits = Intersections::default();
            self.intersect(&ray, &mut hits);

            if hits.has_hit() {
                let comps = self.prepare_comps(hits.get_hit(), &ray);
                colour += self.shade_hit(&comps) * prev_reflectance;

                prev_reflectance = self.shape(comps.obj).material.reflective;
                if prev_reflectance < EPSILON {
                    break;
                }
                ray = Ray {
                    origin: comps.over_point,
                    direction: comps.reflectv,
                };
            } else {
                break;
            }
        }

        colour
    }

    pub fn intersect(&self, ray: &Ray, hits: &mut Intersections) {
        for i in 0..(self.shapes.len() as u32) {
            let shape = self.shape(i);
            shape.intersect(&ray, hits);
        }
    }

    pub fn shade_hit(&self, comps: &Comps) -> Vec3A {
        let sphere = self.shape(comps.obj);
        let mut colour = Vec3A::ZERO;

        for i in 0..self.lights.len() {
            let light = self.light(i);
            let is_shadowed = self.is_shadowed(light, comps.over_point);
            colour += sphere.material.lighting(
                light,
                comps.over_point,
                comps.eyev,
                comps.normalv,
                is_shadowed,
            );
        }

        colour
    }

    pub fn is_shadowed(&self, light: &PointLight, world_point: Vec4) -> bool {
        let light_dir = light.position - world_point;
        let ray = Ray {
            origin: world_point,
            direction: light_dir.normalize(),
        };

        let mut hits = Intersections::default();
        self.intersect(&ray, &mut hits);

        hits.has_hit() && hits.get_hit().t.powi(2) < light_dir.length_squared()
    }

    pub fn prepare_comps(&self, hit: &Intersection, ray: &Ray) -> Comps {
        let point = ray.position(hit.t);
        let eyev = -ray.direction;
        let shape = self.shape(hit.obj);
        let mut normalv = shape.normal_at(point);
        let is_inside = eyev.dot(normalv) < 0.0;
        if is_inside {
            normalv = -normalv;
        }

        let tiny = normalv * EPSILON;

        Comps {
            t: hit.t,
            obj: hit.obj,
            point,
            eyev,
            normalv,
            is_inside,
            over_point: point + tiny,
            reflectv: reflect_vec(ray.direction, normalv),
        }
    }
}

// Avoids bounds checks in the generated spir-v
impl<'a> WorldView<'a> {
    pub fn shape(&self, index: u32) -> &Shape {
        debug_assert!(index < self.shapes.len() as u32);
        unsafe { self.shapes.index_unchecked(index as usize) }
    }

    pub fn light(&self, index: usize) -> &PointLight {
        debug_assert!(index < self.lights.len());
        unsafe { self.lights.index_unchecked(index) }
    }
}
