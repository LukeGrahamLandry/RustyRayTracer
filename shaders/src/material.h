#ifndef material_h
#define material_h

#include "common.h"

struct PointLight;

typedef struct Material {
    float3 colour;
    float ambient;
    float diffuse;
    float specular;
    float shininess;
    float reflective;
    float transparency;
    float refractive_index;
    
    float3 lighting(PointLight light, float4 position, float4 eye_vector, float4 normal_vector, bool in_shadow) const;
} Material;

#include "world.h"

#endif 
