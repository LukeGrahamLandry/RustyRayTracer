use spirv_std::glam::{Vec3A, Vec4};
use spirv_std::num_traits::Pow;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Material {
    // and Vulkan stood upon the hilltop to proclaim the word of the lord,
    // and the word was that all that is good and holy shall be aligned to 16 bytes,
    // even if that seems dumb cause colours are 12 bytes god damn it.
    // think vec3 is 12 on cpu but padded to 16 on gpu to match the spec so always need to use Vec3A to match the padding cpu side
    pub colour: Vec3A,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
}

#[repr(C)]
pub struct PointLight {
    pub position: Vec4,
    pub intensity: Vec3A,
}

impl Material {
    pub fn lighting(
        &self,
        light: &PointLight,
        position: Vec4,
        eye_vector: Vec4,
        normal_vector: Vec4,
        in_shadow: bool,
    ) -> Vec3A {
        let base_colour = self.colour * light.intensity;
        let ambient_colour = self.colour * self.ambient;

        if in_shadow {
            return ambient_colour;
        }

        let light_direction = (light.position - position).normalize();
        let cos_light_to_normal = light_direction.dot(normal_vector); // Since both are normalized

        let mut diffuse_colour = Vec3A::new(0.0, 0.0, 0.0);
        let mut specular_colour = Vec3A::new(0.0, 0.0, 0.0);
        if cos_light_to_normal >= 0.0 {
            diffuse_colour = base_colour * self.diffuse * cos_light_to_normal;

            let reflection_direction = reflect_vec(-light_direction, normal_vector);
            let cos_reflect_to_eye = reflection_direction.dot(eye_vector); // Since both are normalized

            if cos_reflect_to_eye >= 0.0 {
                let factor = cos_reflect_to_eye.pow(self.shininess);
                specular_colour = light.intensity * self.specular * factor;
            }
        }

        ambient_colour + diffuse_colour + specular_colour
    }
}

pub fn reflect_vec(v: Vec4, normal: Vec4) -> Vec4 {
    v - normal * 2.0 * v.dot(normal)
}

impl Default for Material {
    fn default() -> Self {
        Material {
            colour: Vec3A::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
        }
    }
}
