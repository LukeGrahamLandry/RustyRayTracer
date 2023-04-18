#ifndef material_h
#define material_h

#include "common.h"

struct PointLight;

typedef enum PatternType {
    Solid,
    Stripes,
    Gradient,
    Ring,
    Checker
} PatternType;

typedef struct Pattern {
    float3 a;
    float3 b;
    PatternType pattern;
    float4x4 transform_inverse;
} Pattern;

typedef struct Material {
    float3 colour;
    int pattern_index;
    float ambient;
    float diffuse;
    float specular;
    float shininess;
    float reflective;
    float transparency;
    float refractive_index;
    
    float3 lighting(float3 object_colour, PointLight light, float4 position, float4 eye_vector, float4 normal_vector, bool in_shadow) const;
} Material;

#include "world.h"

#endif 
