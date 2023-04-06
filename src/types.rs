use glam::{Mat4, Vec3A, Vec4};

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Camera {
    transform_inverse: Mat4,
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
    hsize: f32,
    vsize: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Material {
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

#[repr(C)]
pub struct Ray {
    pub origin: Vec4,
    pub direction: Vec4,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Intersection {
    pub t: f32,
    pub obj: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Comps {
    pub t: f32,
    pub obj: u32,
    pub point: Vec4,
    pub eyev: Vec4,
    pub normalv: Vec4,
    pub is_inside: bool,
    pub over_point: Vec4,
    pub reflectv: Vec4,
}

pub const MAX_HITS: usize = 100;

#[repr(C)]
pub struct Intersections {
    hits: [Intersection; MAX_HITS],
    count: u32,
    is_hit: bool,
}

// even if this only has one variant and could be zero sized, it MUST be repr(C) and waste space to make it work in storage buffers.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ShapeType {
    Sphere,
    Plane,
}

#[repr(C)]
#[derive(Default)]
pub struct Shape {
    pub transform_inverse: Mat4,
    pub shape: ShapeType,
    // usize is 8 bytes on the cpu but 4 bytes on the gpu so never ever ever use it in structs
    pub index: u32,
    pub material: Material,
}

pub const EPSILON: f32 = 0.01;
pub const REFLECTION_DEPTH: u32 = 5;

pub struct ShaderInputs {
    pub time: f32,
    pub camera: Camera,
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

    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect_ratio = (hsize as f32) / (vsize as f32);
        let half_width = if aspect_ratio >= 1.0 {
            half_view
        } else {
            half_view * aspect_ratio
        };
        let half_height = if aspect_ratio >= 1.0 {
            half_view / aspect_ratio
        } else {
            half_view
        };

        Camera {
            transform_inverse: Mat4::IDENTITY,
            half_width,
            half_height,
            pixel_size: (half_width * 2.0) / (hsize as f32),
            hsize: hsize as f32,
            vsize: vsize as f32,
        }
    }

    pub fn size(&self) -> (f32, f32) {
        (self.hsize, self.vsize)
    }
}
