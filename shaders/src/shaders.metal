#include <metal_stdlib>
#include "material.h"
#include "world.h"
#include "ray.h"
#include "shapes.h"

using namespace metal;

struct VertOut {
    float4 position [[position]];
};

fragment float4 raytracer_fragment(
       VertOut in [[stage_in]],
       constant ShaderInputs& inputs [[buffer(0)]],
       const device Shape* shapes [[buffer(1)]],
       const device PointLight* lights [[buffer(2)]]
){
    World world = {shapes, lights, inputs};
    Ray ray = inputs.camera.ray_for_pixel(in.position.x, in.position.y);
    return float4(world.color_at(ray), 1.0);
};

// Big triangle that covers the screen so the fragment shader runs for every pixel.
// https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
vertex VertOut full_screen_triangle(unsigned int i [[ vertex_id ]]) {
    return { float4(2 * (float) ((i << 1) & 2) - 1, 2 * (float) (i & 2) - 1, 0, 1) };
}
