use std::collections::HashMap;
use glam::{Mat4, Vec3, vec3, Vec3A, vec3a, Vec4};
use yaml_rust::{ScanError, Yaml, YamlLoader};
use yaml_rust::yaml::{Array, Hash};
use crate::bindings::{Camera, PatternType, Shape, ShapeType};
use crate::scene::SceneParseErr::ScanFailed;
use crate::shader_types::{PointLight, World};

/// Switch between these at runtime with the number keys.
pub const SCENE_FILES: &[&str] = &[
    include_str!("../scenes/three-spheres.yml"),
    include_str!("../scenes/puppets.yml"),
    include_str!("../scenes/metal.yml"),
    include_str!("../scenes/reflect-refract.yml"),
    include_str!("../scenes/air-bubble.yml"),
    include_str!("../scenes/table.yml"),
    include_str!("../scenes/cover.yml")
];

#[derive(Debug)]
pub enum SceneParseErr {
    ScanFailed(ScanError),
    InvalidCameraSize
}

#[derive(Default)]
struct ParseContext {
    world: World,
    templates:  HashMap<String, Yaml>
}

// TODO: this would definitely be cleaner with serde but I find the systematic tediousness of doing it manually kinda pleasing.
// TODO: make this give meaningful errors instead of panicking so I can make a REPL.
/// Loads a scene description in the format used on https://forum.raytracerchallenge.com/board/4/gallery?q=scene+description
pub fn load_scene(definition: &str) -> Result<World, SceneParseErr> {
    let data = YamlLoader::load_from_str(definition)?;
    let mut ctx = ParseContext::default();

    if let Yaml::Array(data) = &data[0] {
        for entry in data {
            if let Some(entry) = entry.as_hash() {
                entry.if_str("add", |name| ctx.handle_add(name, entry));
                entry.if_str("define", |name| ctx.handle_define(name, entry));
            }
        }
    }

    ctx.build()
}

impl ParseContext {
    fn handle_add(&mut self, obj_type: &str, entry: &Hash) {
        match obj_type {
            "camera" => self.add_camera(entry),
            "light" => self.add_light(entry),
            "plane" => self.add_shape(entry, ShapeType::Plane),
            "sphere" => self.add_shape(entry, ShapeType::Sphere),
            "cube" => self.add_shape(entry, ShapeType::Cube),
            &_ => {}
        }
    }

    fn handle_define(&mut self, name: &str, entry: &Hash) {
        let mut result = entry.get_any("value").unwrap().clone();

        if let Some(Yaml::String(extend)) = entry.get_any("extend") {
            let current = &result.as_hash().expect("Only type=Hash can extend.");
            result = Yaml::Hash(self.extend_hash_template(current, extend.as_str()))
        }

        if let Yaml::Array(current) = &result {
            result = Yaml::Array(self.include_array_template(current));
        }

        self.templates.insert(name.to_string(), result);
    }

    /// Merges values from current and template into a new Hash. When keys collide, current overrides template.
    fn extend_hash_template(&self, current: &Hash, template_key: &str) -> Hash {
        let prev = self.templates.get(template_key).unwrap();
        let mut prev = prev.as_hash().expect("Hash can only extend type=Hash").clone();
        for (key, value) in current {
            prev.insert(key.clone(), value.clone());
        }
        prev
    }

    /// Checks the `current` for any string entries that are keys for templates. Returns a new Array with those templates evaluated.
    fn include_array_template(&self, current: &Array) -> Array {
        let mut expanded = vec![];
        for value in current {
            if let Yaml::String(key) = &value {  // if its a string it might be a template key
                if let Some(prev) = self.templates.get(key.as_str()) {  // if it was a template key
                    let prev = prev.as_vec().unwrap().clone();  // only arrays can be merged with arrays
                    for value in prev {
                        expanded.push(value);
                    }
                } else {  // it could just be an array of strings. then its ambiguous if one happens to be a template name tho.
                    expanded.push(value.clone());
                }
            } else {
                expanded.push(value.clone());
            }
        }

        expanded
    }

    fn add_shape(&mut self, entry: &Hash, shape: ShapeType) {
        let mut shape = shape.create();
        if let Some(m) = entry.get_any("material") {
            let m = match m {
                Yaml::Hash(m) => m,
                Yaml::String(name) => match self.templates.get(name.as_str()) {
                    Some(Yaml::Hash(m)) => m,
                    _ => panic!("Undefined material name: {:?}", m),
                },
                _ => panic!("Invalid material value: {:?}", m),
            }.clone();

            self.parse_material(&m, &mut shape);
        }

        if let Some(&Yaml::Boolean(shadow)) = entry.get_any("shadow") {
            if shadow {
                // TODO: let shapes opt out of casting shadows.
            }
        }

        self.if_transform(entry, |t| shape.set_transform(t));
        self.world.add_shape(shape);
    }

    fn parse_material(&mut self, m_obj: &Hash, shape: &mut Shape) {
        m_obj.if_f32("diffuse", |v| shape.material.diffuse = v);
        m_obj.if_f32("ambient", |v| shape.material.ambient = v);
        m_obj.if_f32("specular", |v| shape.material.specular = v);
        m_obj.if_f32("shininess", |v| shape.material.shininess = v);
        m_obj.if_f32("reflective", |v| shape.material.reflective = v);
        m_obj.if_colour("color", |v| shape.material.colour = v);
        m_obj.if_map("pattern", |p| self.parse_pattern(p, shape));
    }

    fn parse_pattern(&mut self, p_obj: &Hash, shape: &mut Shape) {
        let data = p_obj.get_any("colors").unwrap().as_vec().unwrap();
        let mut pattern = get_pattern_type(&p_obj.get_str("type")).create();
        pattern.a = to_colour(data[0].as_vec().unwrap());
        pattern.b = to_colour(data[1].as_vec().unwrap());
        self.if_transform(p_obj, |t| pattern.set_transform(t));
        shape.material.pattern_index = self.world.add_pattern(pattern);
    }

    fn if_transform(&self, obj: &Hash, action: impl FnOnce(Mat4)) {
        if let Some(Yaml::Array(t)) = obj.get_any("transform") {
            action(self.parse_transform(&t));
        }
    }

    // unlike material templates, the transformation lists on shapes sometimes use a template but add extra in place,
    // so this always checks if there are any templates to expand.
    fn parse_transform(&self, t: &Array) -> Mat4 {
        let t = self.include_array_template(t);
        let mut transform = Mat4::IDENTITY;
        for part in t {
            transform = to_mat(&part) * transform;
        }
        transform
    }

    fn add_light(&mut self, entry: &Hash) {
        self.world.add_light(PointLight {
            position: entry.get_point("at"),
            intensity: entry.get_colour("intensity"),
        })
    }

    fn add_camera(&mut self, entry: &Hash) {
        self.world.camera = Camera::new(entry.get_usize("width"), entry.get_usize("height"), entry.get_f32("field-of-view"));
        self.world.camera.set_transform(Mat4::look_at_rh(
            entry.get_vec3("from"),
            entry.get_vec3("to"),
            entry.get_vec3("up"),
        ))
    }

    fn build(self) -> Result<World, SceneParseErr> {
        if self.world.camera.vsize <= 0.0 || self.world.camera.hsize <= 0.0 {
            Err(SceneParseErr::InvalidCameraSize)
        } else {
            Ok(self.world)
        }
    }
}


trait AssertMap<'a> {
    fn get_any(self, key: &str) -> Option<&'a Yaml>;
    fn if_str(self, key: &str, action: impl FnOnce(&str));
    fn if_map(self, key: &str, action: impl FnOnce(&Hash));
    fn get_f32(self, key: &str) -> f32;
    fn get_usize(self, key: &str) -> usize;
    fn get_vec3(self, key: &str) -> Vec3;
    fn get_point(self, key: &str) -> Vec4;
    fn get_colour(self, key: &str) -> Vec3A;
    fn get_str(self, key: &str) -> String;
    fn if_f32(self, key: &str, action: impl FnMut(f32));
    fn if_colour(self, key: &str, action: impl FnMut(Vec3A));
}

impl<'a> AssertMap<'a> for &'a Hash {
    fn get_any(self, key: &str) -> Option<&'a Yaml> {
        let key = Yaml::String(key.to_string());
        self.get(&key)
    }

    fn if_str(self, key: &str, action: impl FnOnce(&str)) {
        if let Some(value) = self.get_any(key) {
            let value = value.as_str().unwrap();
            action(value);
        }
    }

    fn if_map(self, key: &str, action: impl FnOnce(&Hash)) {
        if let Some(value) = self.get_any(key) {
            let value = value.as_hash().unwrap();
            action(value);
        }
    }

    fn get_f32(self, key: &str) -> f32 {
        to_f32(self.get_any(key).unwrap())
    }

    fn get_usize(self, key: &str) -> usize {
        self.get_any(key).unwrap().as_i64().unwrap() as usize
    }

    fn get_vec3(self, key: &str) -> Vec3 {
        let data = self.get_any(key).unwrap().as_vec().unwrap();
        vec3(to_f32(&data[0]), to_f32(&data[1]), to_f32(&data[2]))
    }

    fn get_point(self, key: &str) -> Vec4 {
        self.get_vec3(key).extend(1.0)
    }

    fn get_colour(self, key: &str) -> Vec3A {
        self.get_vec3(key).into()
    }

    fn get_str(self, key: &str) -> String {
        self.get_any(key).unwrap().as_str().unwrap().to_string()
    }

    fn if_f32(self, key: &str, mut action: impl FnMut(f32)) {
        if let Some(data) = self.get_any(key) {
            if let Some(v) = maybe_f32(data) {
                action(v);
            }
        }
    }

    fn if_colour(self, key: &str, mut action: impl FnMut(Vec3A)) {
        if let Some(Yaml::Array(data)) = self.get_any(key) {
            action(vec3a(to_f32(&data[0]), to_f32(&data[1]), to_f32(&data[2])))
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
        "rotate-x" => Mat4::from_rotation_x(to_f32(&data[1])),
        "rotate-y" => Mat4::from_rotation_y(to_f32(&data[1])),
        "rotate-z" => Mat4::from_rotation_z(to_f32(&data[1])),
        &_ => panic!("Invalid transform {:?}", yaml),
    }
}

fn offset_vec3(data: &Array) -> Vec3 {
    vec3(to_f32(&data[1]), to_f32(&data[2]), to_f32(&data[3]))
}

fn maybe_f32(yaml: &Yaml) -> Option<f32> {
    match yaml {
        Yaml::Real(_) => Some(yaml.as_f64().unwrap() as f32),
        Yaml::Integer(v) => Some(*v as f32),
        _ => None
    }
}

fn get_pattern_type(name: &str) -> PatternType {
    match name {
        "stripes" => PatternType::Stripes,
        "checkers" => PatternType::Checker,
        &_ => panic!("Invalid pattern type: {}", name),
    }
}

impl From<ScanError> for SceneParseErr {
    fn from(value: ScanError) -> Self {
        ScanFailed(value)
    }
}
