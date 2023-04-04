use spirv_std::glam::Vec3A;
use crate::material::PointLight;
use crate::ray::{Comps, Intersection, Intersections, Ray};
use crate::shapes::Shape;

pub struct WorldView<'a> {
    pub shapes: &'a [Shape],
    pub lights: &'a [PointLight]
}

impl<'a> WorldView<'a> {
    pub fn color_at(&self, ray: &Ray) -> Vec3A {
        let hits = self.intersect(ray);
        if hits.has_hit() {
            let comps = self.prepare_comps(hits.get_hit(), ray);
            self.shade_hit(&comps)
        } else {
            Vec3A::ZERO
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut hits = Intersections::default();

        for i in 0..self.shapes.len() {
            self.shapes[i].intersect(&ray, &mut hits);
        }

        hits
    }

    pub fn shade_hit(&self, comps: &Comps) -> Vec3A {
        let sphere = &self.shapes[comps.obj as usize];
        let mut colour = Vec3A::ZERO;

        for i in 0..self.lights.len() {
            colour += sphere.material.lighting(
                &self.lights[i],
                comps.point,
                comps.eyev,
                comps.normalv,
                false,
            );
        }

        colour
    }

    pub fn prepare_comps(&self, hit: &Intersection, ray: &Ray) -> Comps {
        let point = ray.position(hit.t);
        let eyev = -ray.direction;
        let mut normalv = self.shapes[hit.obj as usize].normal_at(point);
        let is_inside = eyev.dot(normalv) < 0.0;
        if is_inside {
            normalv = -normalv;
        }

        Comps {
            t: hit.t,
            obj: hit.obj,
            point,
            eyev,
            normalv,
            is_inside
        }
    }
}