use glam::{Mat4, Vec3A, Vec4};

#[derive(Default)]
pub struct World {
    shapes: Vec<Shape>,
    lights: Vec<PointLight>,
    pub camera: Camera,
}

impl World {
    pub fn add_shape(&mut self, mut shape: Shape) {
        debug_assert!(is_frac(shape.material.diffuse)
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

    pub fn get_shapes(&self) -> &[Shape] {
        self.shapes.as_slice()
    }

    pub fn get_lights(&self) -> &[PointLight] {
        self.lights.as_slice()
    }
}

fn is_frac(x: f32) -> bool {
    0.0 <= x && x <= 1.0
}

fn is_colour(c: Vec3A) -> bool {
    is_frac(c.x) && is_frac(c.y) && is_frac(c.z)
}

// The structs below are used to pass information to the shaders.
// It's important that they are all repr(C) and that field orders match.
// They could be copy but it feels like I might accidentally modify after adding a copy to the world.

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Camera {
    transform_inverse: Mat4,
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
    hsize: f32,
    vsize: f32,
    field_of_view: f32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Material {
    pub colour: Vec3A,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
    pub transparency: f32,
    pub refractive_index: f32
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct PointLight {
    pub position: Vec4,
    pub intensity: Vec3A,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ShapeType {
    Sphere,
    Plane,
}

#[repr(C)]  // Clone works because the world sets the index to be correct
#[derive(Clone, Default, Debug)]
pub struct Shape {
    pub transform_inverse: Mat4,
    pub shape: ShapeType,
    // usize is 8 bytes on the cpu but 4 bytes on the gpu so never ever ever use it in structs
    pub index: u32,
    pub material: Material,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct ShaderInputs {
    pub time: f32,
    pub camera: Camera,
    pub shape_count: u32,
    pub light_count: u32
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
            transparency: 0.0,
            refractive_index: 1.0
        }
    }
}

impl Default for ShapeType {
    fn default() -> Self {
        ShapeType::Sphere
    }
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
        let mut camera = Camera::default();
        camera.field_of_view = field_of_view;
        camera.set_transform(Mat4::IDENTITY);
        camera.resize(hsize, vsize);
        camera
    }

    #[no_mangle]
    pub extern fn resize(&mut self, hsize: usize, vsize: usize) {
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

#[cfg(test)]
extern {
    fn run_tests() -> i32;
}

#[test]
fn cc_tests(){
    unsafe {
        run_tests();
    }
}
