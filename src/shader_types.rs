pub use crate::bindings::*;
use glam::{Mat4, Vec3A};

#[derive(Default)]
pub struct World {
    pub shapes: Vec<Shape>,
    pub lights: Vec<PointLight>,
    pub patterns: Vec<Pattern>,
    pub camera: Camera,
}

impl World {
    pub fn add_shape(&mut self, mut shape: Shape) {
        debug_assert!(
            is_frac(shape.material.diffuse)
                && is_frac(shape.material.ambient)
                && is_frac(shape.material.reflective)
                && is_frac(shape.material.specular)
                && is_frac(shape.material.transparency)
                && is_colour(shape.material.colour)
                && shape.material.refractive_index >= 0.0
        );

        shape.index = self.shapes.len() as u32;
        self.shapes.push(shape);
    }

    pub fn add_light(&mut self, light: PointLight) {
        debug_assert!(is_colour(light.intensity));
        self.lights.push(light);
    }

    pub fn add_pattern(&mut self, pattern: Pattern) -> i32 {
        self.patterns.push(pattern);
        (self.patterns.len() - 1) as i32
    }

    pub fn get_shapes(&self) -> &[Shape] {
        self.shapes.as_slice()
    }

    pub fn get_lights(&self) -> &[PointLight] {
        self.lights.as_slice()
    }

    pub fn get_patterns(&self) -> &[Pattern] {
        self.patterns.as_slice()
    }

    pub fn view(&self) -> WorldView {
        WorldView {
            shapes: self.shapes.as_ptr(),
            lights: self.lights.as_ptr(),
            patterns: self.patterns.as_ptr(),
            __bindgen_padding_0: 0,
            inputs: ShaderInputs {
                camera: self.camera,
                shape_count: self.shapes.len() as u32,
                light_count: self.lights.len() as u32,
            },
        }
    }
}

// Fight me clippy. There's no universe where (0.0..=1.0).contains(&x) is better.
#[allow(clippy::manual_range_contains)]
fn is_frac(x: f32) -> bool {
    0.0 <= x && x <= 1.0
}

fn is_colour(c: Vec3A) -> bool {
    is_frac(c.x) && is_frac(c.y) && is_frac(c.z)
}

impl Shape {
    pub fn set_transform(&mut self, mat: Mat4) {
        self.transform_inverse = mat.inverse();
    }
}

impl Camera {
    pub fn set_transform(&mut self, mat: Mat4) {
        self.transform_inverse = mat.inverse();
    }

    pub fn get_transform(&self) -> Mat4 {
        self.transform_inverse.inverse()
    }

    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        let mut camera = Camera {
            field_of_view,
            ..Default::default()
        };
        camera.set_transform(Mat4::IDENTITY);
        camera.resize(hsize, vsize);
        camera
    }

    pub fn resize(&mut self, hsize: usize, vsize: usize) {
        let half_view = (self.field_of_view / 2.0).tan();
        let aspect_ratio = (hsize as f32) / (vsize as f32);
        self.half_width = if aspect_ratio >= 1.0 {
            half_view
        } else {
            half_view * aspect_ratio
        };
        self.half_height = if aspect_ratio >= 1.0 {
            half_view / aspect_ratio
        } else {
            half_view
        };

        self.pixel_size = (self.half_width * 2.0) / (hsize as f32);
        self.hsize = hsize as f32;
        self.vsize = vsize as f32;
    }

    pub fn size(&self) -> (f32, f32) {
        (self.hsize, self.vsize)
    }
}

impl Pattern {
    pub fn set_transform(&mut self, mat: Mat4) {
        self.transform_inverse = mat.inverse();
    }
}


// I don't accept pointers being !Sync just because they can be made into mut ones.
// It's unsafe to dereference them anyway so that's a you problem.
// If something's only unsafe in unsafe code... that's safe. Or I'm just dumb and missing something here.
// When passed to the c code, it never mutates.
unsafe impl Sync for WorldView {}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            transform_inverse: Default::default(),
            pixel_size: 0.0,
            half_width: 0.0,
            half_height: 0.0,
            hsize: 0.0,
            vsize: 0.0,
            field_of_view: 0.0,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            colour: Vec3A::new(1.0, 1.0, 1.0),
            pattern_index: -1,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}

impl ShapeType {
    pub(crate) fn create(self) -> Shape {
        Shape {
            transform_inverse: Default::default(),
            shape: self,
            index: 0,
            __bindgen_padding_0: 0,
            material: Default::default(),
        }
    }
}

impl Copy for Material {}
impl Clone for Material {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Camera {}
impl Clone for Camera {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Pattern {}
impl Clone for Pattern {
    fn clone(&self) -> Self {
        *self
    }
}
