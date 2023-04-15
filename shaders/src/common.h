#ifndef common_h
#define common_h

// Used for the size of the static arrays of intersections.
#define MAX_HITS 100

// ~log_2(MAX_REFLECT_REFRACT)? or maybe MAX_REFLECT_REFRACT/2
// Used for avoiding recursion in the colour_at function
#define MAX_RAY_QUEUE 5

// TODO: this is just used for the loop counter so could be passed in at runtime.
#define MAX_REFLECT_REFRACT 10

// Used for preventing shadow acne.
#define EPSILON 0.01

#define unreachable() __builtin_unreachable();
#define BLACK() float3(0)

// NOT_BUILDING_AS_MSL is set by build.rs when building as c++ for cpu_runner.
#ifdef NOT_BUILDING_AS_MSL

// bindgen can't find simd and if you give it the include-path it chokes on a bunch of other stuff.
// But I want to use the glam types anyway so who cares.
#ifndef DOING_RUST_BINDGEN
#include <simd/simd.h>
using namespace simd;
#endif

#define Point(x, y, z) simd_make_float4(x, y, z, 1.0)
#define Vector(x, y, z) simd_make_float4(x, y, z, 0.0)

// These are address qualifiers in MSL. Define them as macros that expand to an empty string.
// That's all it takes to compile it as c++.
#define device
#define constant
#define thread

#else

#include <metal_stdlib>
using namespace metal;
#define Point(x, y, z) float4(x, y, z, 1.0)
#define Vector(x, y, z) float4(x, y, z, 0.0)

#endif
#endif
