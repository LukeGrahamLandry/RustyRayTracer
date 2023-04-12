#ifndef common_h
#define common_h

#include <metal_stdlib>
using namespace metal;

#define Point(x, y, z) float4(x, y, z, 1.0)
#define Vector(x, y, z) float4(x, y, z, 0.0)

// Used for the size of the static arrays of intersections.
#define MAX_HITS 100
#define MAX_RAY_QUEUE 10

// TODO: this is just used for the loop counter so could be passed in at runtime.
#define MAX_REFLECTIONS 5

// Used for preventing shadow acne.
#define EPSILON 0.01

#endif
