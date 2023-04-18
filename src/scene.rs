use glam::{Mat4, Vec3, vec3, Vec3A, vec3a, Vec4, vec4};
use yaml_rust::{ScanError, Yaml, YamlLoader};
use yaml_rust::yaml::{Array, Hash};
use crate::bindings::{Camera, PatternType, ShapeType};
use crate::scene::SceneParseErr::ScanFailed;
use crate::shader_types::{PointLight, World};

pub const SCENE_FILES: &[&str] = &[
    include_str!("../scenes/metal.yml")
];

#[derive(Debug)]
pub enum SceneParseErr {
    ScanFailed(ScanError),
    InvalidCameraSize,
    InvalidData
}

// TODO: this would definitely be cleaner with serde but I find the systematic tediousness of doing it manually kinda pleasing.
/// Loads a scene description in the format used on https://forum.raytracerchallenge.com/board/4/gallery?q=scene+description
pub fn load_scene(definition: &str) -> Result<World, SceneParseErr> {
    let data = YamlLoader::load_from_str(definition)?;
    let mut world = World::default();

    let add = &Yaml::String(String::from("add"));
    match data[0].as_vec() {
        None => {
            return Err(SceneParseErr::InvalidData);
        }
        Some(data) => {
            for entry in data {
                let entry = match entry.as_hash() {
                    None => continue,
                    Some(e) => e
                };

                let obj_type = entry.get(add).unwrap().as_str().unwrap();
                match obj_type {
                    "camera" => add_camera(entry, &mut world),
                    "light" => add_light(entry, &mut world),
                    "plane" => add_shape(entry, &mut world, ShapeType::Plane),
                    "sphere" => add_shape(entry, &mut world, ShapeType::Sphere),
                    &_ => {}
                }
            }
        }
    }

    if world.camera.vsize <= 0.0 || world.camera.hsize <= 0.0 {
        Err(SceneParseErr::InvalidCameraSize)
    } else {
        Ok(world)
    }

}

fn add_shape(entry: &Hash, world: &mut World, shape: ShapeType) {
    let mut shape = shape.create();

    match entry.get(&Yaml::String(String::from("material"))) {
        None => {}
        Some(m) => {
            let m = m.as_hash().unwrap();
            m.if_f32("diffuse", |v| shape.material.diffuse = v);
            m.if_f32("ambient", |v| shape.material.ambient = v);
            m.if_f32("specular", |v| shape.material.specular = v);
            m.if_f32("shininess", |v| shape.material.shininess = v);
            m.if_f32("reflective", |v| shape.material.reflective = v);
            m.if_colour("color", |v| shape.material.colour = v);
            match m.get(&Yaml::String(String::from("pattern"))) {
                None => {}
                Some(p) => {
                    let p = p.as_hash().unwrap();
                    let key = &Yaml::String(String::from("colors"));
                    let data = p.get(key).unwrap().as_vec().unwrap();
                    let mut pattern = get_pattern_type(&p.get_str("type")).create();
                    pattern.a = to_colour(data[0].as_vec().unwrap());
                    pattern.b = to_colour(data[1].as_vec().unwrap());
                    p.if_transform(|t| pattern.set_transform(t));
                    shape.material.pattern_index = world.add_pattern(pattern);
                }
            }
        }
    }

    entry.if_transform(|t| shape.set_transform(t));
    world.add_shape(shape);
}

fn add_light(entry: &Hash, world: &mut World) {
    world.add_light(PointLight {
        position: entry.get_point("at"),
        intensity: entry.get_colour("intensity"),
    })
}

fn add_camera(entry: &Hash, world: &mut World) {
    world.camera = Camera::new(entry.get_usize("width"), entry.get_usize("height"), entry.get_f32("field-of-view"));
    world.camera.set_transform(Mat4::look_at_rh(
        entry.get_vec3("from"),
        entry.get_vec3("to"),
        entry.get_vec3("up"),
    ))
}

trait AssertMap {
    fn get_f32(self, key: &str) -> f32;
    fn get_usize(self, key: &str) -> usize;
    fn get_vec3(self, key: &str) -> Vec3;
    fn get_point(self, key: &str) -> Vec4;
    fn get_colour(self, key: &str) -> Vec3A;
    fn get_str(self, key: &str) -> String;
    fn if_f32(self, key: &str, action: impl FnMut(f32));
    fn if_colour(self, key: &str, action: impl FnMut(Vec3A));
    fn if_transform(self, action: impl FnMut(Mat4));
}

impl AssertMap for &Hash {
    fn get_f32(self, key: &str) -> f32 {
        let key = &Yaml::String(String::from(key));
        to_f32(self.get(key).unwrap())
    }

    fn get_usize(self, key: &str) -> usize {
        let key = &Yaml::String(String::from(key));
        self.get(key).unwrap().as_i64().unwrap() as usize
    }

    fn get_vec3(self, key: &str) -> Vec3 {
        let key = &Yaml::String(String::from(key));
        let data = self.get(key).unwrap().as_vec().unwrap();
        vec3(to_f32(&data[0]), to_f32(&data[1]), to_f32(&data[2]))
    }

    fn get_point(self, key: &str) -> Vec4 {
        let key = &Yaml::String(String::from(key));
        let data = self.get(key).unwrap().as_vec().unwrap();
        vec4(to_f32(&data[0]), to_f32(&data[1]), to_f32(&data[2]), 1.0)
    }

    fn get_colour(self, key: &str) -> Vec3A {
        let key = &Yaml::String(String::from(key));
        let data = self.get(key).unwrap().as_vec().unwrap();
        to_colour(data)
    }

    fn get_str(self, key: &str) -> String {
        let key = &Yaml::String(String::from(key));
        self.get(key).unwrap().as_str().unwrap().to_string()
    }

    fn if_f32(self, key: &str, mut action: impl FnMut(f32)) {
        let key = &Yaml::String(String::from(key));
        match self.get(key) {
            None => {}
            Some(data) => {
                match maybe_f32(data) {
                    None => {}
                    Some(v) => action(v),
                }
            }
        }
    }

    fn if_colour(self, key: &str, mut action: impl FnMut(Vec3A)) {
        let key = &Yaml::String(String::from(key));
        if let Some(Yaml::Array(data)) = self.get(key) {
            action(vec3a(to_f32(&data[0]), to_f32(&data[1]), to_f32(&data[2])))
        }
    }

    fn if_transform(self, mut action: impl FnMut(Mat4)) {
        match self.get(&Yaml::String(String::from("transform"))) {
            None => {}
            Some(t) => {
                let mut transform = Mat4::IDENTITY;
                for part in t.as_vec().unwrap() {
                    transform = to_mat(part) * transform;
                }
                action(transform);
            }
        }
    }
}

fn to_colour(data: &Array) -> Vec3A {
    vec3a(to_f32(&data[0]), to_f32(&data[1]), to_f32(&data[2]))
}

fn to_f32(yaml: &Yaml) -> f32 {
    maybe_f32(yaml).unwrap()
}

fn to_mat(yaml: &Yaml) -> Mat4 {
    let data = yaml.as_vec().unwrap();
    let kind = data[0].as_str().unwrap();
    match kind {
        "translate" => Mat4::from_translation(offset_vec3(data)),
        "scale" => Mat4::from_scale(offset_vec3(data)),
        &_ => panic!("Invalid transform {:?}", yaml),
    }
}

fn offset_vec3(data: &Array) -> Vec3 {
    vec3(to_f32(&data[1]), to_f32(&data[2]), to_f32(&data[3]))
}

fn maybe_f32(yaml: &Yaml) -> Option<f32> {
    match yaml {
        Yaml::Real(_) => Some(yaml.as_f64().unwrap() as f32),
        Yaml::Integer(_) => Some(yaml.as_i64().unwrap() as f32),
        _ => None
    }
}

fn get_pattern_type(name: &str) -> PatternType {
    match name {
        "checkers" => PatternType::Checker,
        &_ => panic!("Invalid pattern type: {}", name),
    }
}

impl From<ScanError> for SceneParseErr {
    fn from(value: ScanError) -> Self {
        ScanFailed(value)
    }
}
