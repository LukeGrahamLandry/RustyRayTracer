#ifdef NOT_BUILDING_AS_MSL

#include "common.h"
#include "material.h"
#include "world.h"
#include "ray.h"
#include "shapes.h"

extern "C" {
    // This gets called from cpu_runner.
    void trace_pixel(float4 position, ShaderInputs& inputs, Shape* shapes, PointLight* lights, float4* res){
        World world = World(shapes, lights, inputs);
        Ray ray = inputs.camera.ray_for_pixel(position.x, position.y);
        float3 colour = world.colour_at(ray);
        *res = make_float4(colour.x, colour.y, colour.z, 1.0);
    };
}

#endif
