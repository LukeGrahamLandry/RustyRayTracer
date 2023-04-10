#ifndef material_h
#define material_h

#include "common.h"

typedef struct Material {
    float3 color;
    float ambient;
    float diffuse;
    float specular;
    float shininess;
} Material;


#endif 
