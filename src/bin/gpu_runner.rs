extern crate raytracer;

use raytracer::demo::chapter7;

pub fn main() {
    let world = chapter7();
    let (mut state, event_loop) = raytracer::gpu::setup::AppState::new(world);
    event_loop.run(move |event, _, control_flow| {
        state.tick(event, control_flow);
    });
}
