#ifdef DOING_RUST_BINDGEN

struct alignas(16) float4 {
    float data[4];
    float4(float x);
};
struct alignas(16) float3 {
    float data[4];
    float3(float x);
};
struct alignas(16) float4x4 {
    float data[16];
};
float4 simd_make_float4(float x, float y, float z, float w);
typedef int uint32_t;

#include "common.h"
#include "material.h"
#include "world.h"
#include "ray.h"
#include "shapes.h"

void trace_pixel(float4 position, const WorldView& world, float4* res);
#endif
