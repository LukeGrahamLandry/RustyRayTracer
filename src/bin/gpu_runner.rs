extern crate raytracer;

pub fn main() {
    let (mut state, event_loop) = raytracer::gpu::setup::AppState::new();
    event_loop.run(move |event, _, control_flow| {
        state.tick(event, control_flow);
    });
}
