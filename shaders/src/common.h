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

// NOT_BUILDING_AS_MSL is set by build.rs when building as c++ for cpu_runner.
#ifdef NOT_BUILDING_AS_MSL
#include "la.h"

// These are address qualifiers in MSL. Define them as macros that expand to an empty string.
// That's all it takes to compile it as c++.
#define device
#define constant
#define thread

#else

#include <metal_stdlib>
using namespace metal;

#endif
#endif

#define Point(x, y, z) float4(x, y, z, 1.0)
#define Vector(x, y, z) float4(x, y, z, 0.0)
#define BLACK() float3(0.0, 0.0, 0.0)
#define ZERO_VEC() float4(0.0, 0.0, 0.0, 0.0)
