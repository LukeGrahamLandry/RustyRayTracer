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

#include "la.h"

// NOT_BUILDING_AS_MSL is set by build.rs when building as c++ for cpu_runner.
#ifdef NOT_BUILDING_AS_MSL

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
