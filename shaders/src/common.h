#ifndef common_h
#define common_h

// Used for the size of the static arrays of intersections.
#define MAX_HITS 50

// ~log_2(MAX_REFLECT_REFRACT)?
// Used for avoiding recursion in the colour_at function
#define MAX_RAY_QUEUE 5

// TODO: this is just used for the loop counter so could be passed in at runtime.
#define MAX_REFLECT_REFRACT 10

// Used for preventing shadow acne.
#define EPSILON 0.01

#define unreachable() __builtin_unreachable();

// NOT_BUILDING_AS_MSL is set by build.rs when building as c++ for cpu_runner.
#ifndef NOT_BUILDING_AS_MSL

#include <metal_stdlib>
using namespace metal;
#define Point(x, y, z) float4(x, y, z, 1.0)
#define Vector(x, y, z) float4(x, y, z, 0.0)

#else

#include <simd/simd.h>
using namespace simd;
#define Point(x, y, z) simd_make_float4(x, y, z, 1.0)
#define Vector(x, y, z) simd_make_float4(x, y, z, 0.0)
#define device
#define constant
#define thread
#define vertex
#define fragment

#endif
#endif
