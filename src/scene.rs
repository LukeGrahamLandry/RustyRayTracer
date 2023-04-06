use shaders::camera::Camera;
use shaders::material::PointLight;
use shaders::shapes::Shape;

pub struct World {
    pub shapes: Vec<Shape>,
    pub lights: Vec<PointLight>,
    pub camera: Camera,
}

impl World {
    /// Checks that the ids are correct indexes into the arrays.
    pub fn assert_sanity(&mut self) {
        for (i, shape) in self.shapes.iter_mut().enumerate() {
            shape.index = i as u32;

            debug_assert!(
                frac(shape.material.diffuse)
                    && frac(shape.material.ambient)
                    && frac(shape.material.reflective)
                    && frac(shape.material.specular)
            )
        }
    }
}

fn frac(x: f32) -> bool {
    0.0 <= x && x <= 1.0
}
