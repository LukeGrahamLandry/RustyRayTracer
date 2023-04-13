
#ifdef NOT_BUILDING_AS_MSL

#include "common.h"
#include "material.h"
#include "world.h"
#include "ray.h"
#include "shapes.h"

extern "C" {
    float4 trace_pixel(float4 position, ShaderInputs& inputs, Shape* shapes, PointLight* lights){
        World world = World(shapes, lights, inputs);
        Ray ray = inputs.camera.ray_for_pixel(position.x, position.y);
        float3 colour = world.colour_at(ray);
        return simd_make_float4(colour, 1.0);
    };
}

#endif
