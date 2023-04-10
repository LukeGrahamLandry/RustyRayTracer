#include <metal_stdlib>
#include "material.metal"

using namespace metal;


struct VertOut {
    float4 position [[position]];
};


// Big triangle that covers the screen so the fragment shader runs for every pixel.
// https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
vertex VertOut full_screen_triangle(unsigned int i [[ vertex_id ]]) {
    return { float4(2 * (float) ((i << 1) & 2) - 1, 2 * (float) (i & 2) - 1, 0, 1) };
}

fragment float4 raytracer_fragment(VertOut in [[stage_in]]){
    return float4(in.position.x / 800, in.position.y / 600, 0.0, 1.0);
};
