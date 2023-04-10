#ifndef shapes_h
#define shapes_h

#include "common.h"
#include "material.h"

typedef enum ShapeType {
    Sphere,
} ShapeType;

typedef struct Shape {
    float4x4 transform_inverse;
    ShapeType shape;
    int index;
    Material material;
} Shape;

#endif
