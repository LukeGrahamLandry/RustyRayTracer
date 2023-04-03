use spirv_std::glam::{Vec3, vec3, Vec4};
use spirv_std::num_traits::Pow;

pub struct Material {
    pub colour: Vec3,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

pub struct PointLight {
    pub position: Vec4,
    pub intensity: Vec3,
}

impl Material {
    pub fn lighting(&self, light: PointLight, position: Vec4, eye_vector: Vec4, normal_vector: Vec4, in_shadow: bool) -> Vec3 {
        let base_colour = self.colour * light.intensity;
        let ambient_colour = self.colour * self.ambient;

        if in_shadow { return ambient_colour; }

        let light_direction = (light.position - position).normalize();
        let cos_light_to_normal = light_direction.dot(normal_vector);  // Since both are normalized

        let mut diffuse_colour = vec3(0.0, 0.0, 0.0);
        let mut specular_colour = vec3(0.0, 0.0, 0.0);
        if cos_light_to_normal >= 0.0 {
            diffuse_colour = base_colour * self.diffuse * cos_light_to_normal;

            let reflection_direction = reflect_vec(-light_direction, normal_vector);
            let cos_reflect_to_eye = reflection_direction.dot(eye_vector);  // Since both are normalized

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
            colour: vec3(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}