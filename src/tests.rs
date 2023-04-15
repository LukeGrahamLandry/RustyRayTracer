use std::slice;
use std::mem::{align_of, size_of};
use glam::{Mat4, vec3, vec3a, vec4};
use crate::shader_types::{Camera, Material, PointLight, ShaderInputs, Shape, ShapeType, World, WorldView};

#[cfg(test)]
extern {
    fn run_tests() -> i32;
}

#[test]
fn cc_tests(){
    unsafe {
        assert_eq!(run_tests(), 0);
    }
}

#[no_mangle]
extern fn default_world(out: &mut WorldView) {
    let mut world = World::default();
    let mut sphere = Shape::default();
    sphere.material.colour = vec3a(0.8, 1.0, 0.6);
    sphere.material.diffuse = 0.7;
    sphere.material.specular = 0.2;
    world.add_shape(sphere);
    let mut sphere = Shape::default();
    sphere.set_transform(Mat4::from_scale(vec3(0.5, 0.5, 0.5)));
    world.add_shape(sphere);
    world.add_light(PointLight {
        position: vec4(-10.0, 10.0, -10.0, 0.0),
        intensity: vec3a(1.0, 1.0, 1.0),
    });

    *out = world.view()
}

// TODO: figure out how to make a macro i can just put on the struct def, cause this seems dumb.
#[no_mangle]
extern fn get_structs_repr(count: usize, sizes_out: *mut usize, alignment_out: *mut usize) {
    assert_eq!(count, 8);
    let sizes_out = unsafe {
        slice::from_raw_parts_mut(sizes_out, count)
    };
    let alignment_out = unsafe {
        slice::from_raw_parts_mut(alignment_out, count)
    };

    sizes_out[0] = size_of::<Camera>();
    sizes_out[1] = size_of::<Material>();
    sizes_out[2] = size_of::<PointLight>();
    sizes_out[3] = size_of::<ShapeType>();
    sizes_out[4] = size_of::<Shape>();
    sizes_out[5] = size_of::<ShaderInputs>();
    sizes_out[6] = size_of::<WorldView>();
    sizes_out[7] = size_of::<Mat4>();

    alignment_out[0] = align_of::<Camera>();
    alignment_out[1] = align_of::<Material>();
    alignment_out[2] = align_of::<PointLight>();
    alignment_out[3] = align_of::<ShapeType>();
    alignment_out[4] = align_of::<Shape>();
    alignment_out[5] = align_of::<ShaderInputs>();
    alignment_out[6] = align_of::<WorldView>();
    alignment_out[7] = align_of::<Mat4>();
}
