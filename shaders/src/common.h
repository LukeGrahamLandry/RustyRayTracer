#ifndef common_h
#define common_h

#include <metal_stdlib>
using namespace metal;

#define Point(x, y, z) float4(x, y, z, 1.0)
#define Vector(x, y, z) float4(x, y, z, 0.0)

#define MAX_HITS 10
#define EPSILON 0.01

#endif
