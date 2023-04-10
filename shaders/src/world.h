#ifndef world_h
#define world_h

#include "common.h"
#include "shapes.h"

typedef struct PointLight {
    float4 position;
    float3 intensity;
} PointLight;

typedef struct WorldView {
    const device Shape* shapes;
    const device PointLight* lights;
} WorldView;

#endif 
