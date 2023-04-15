#include "common.h"
#include "material.h"
#include "world.h"
#include "ray.h"

#ifdef NOT_BUILDING_AS_MSL

extern "C" {
    // This gets called from cpu_runner.
    void trace_pixel(float4 position, const World& world, float4* res){
        Ray ray = world.inputs.camera.ray_for_pixel(position.x, position.y);
        float3 colour = world.colour_at(ray);
        *res = make_float4(colour.x, colour.y, colour.z, 1.0);
    };
}

#else

struct VertOut {
    float4 position [[position]];
};

// This gets called from gpu_runner.
fragment float4 trace_pixel(
    VertOut in [[stage_in]],
    constant ShaderInputs& inputs [[buffer(0)]],
    const device Shape* shapes [[buffer(1)]],
    const device PointLight* lights [[buffer(2)]]
){
    World world = World(shapes, lights, inputs);
    Ray ray = inputs.camera.ray_for_pixel(in.position.x, in.position.y);
    return float4(world.colour_at(ray), 1.0);
};

// Big triangle that covers the screen so the fragment shader runs for every pixel.
// https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
vertex VertOut full_screen_triangle(unsigned int i [[ vertex_id ]]) {
    return { float4(2 * (float) ((i << 1) & 2) - 1, 2 * (float) (i & 2) - 1, 0, 1) };
}

#endif
