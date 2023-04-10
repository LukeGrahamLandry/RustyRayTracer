#ifndef shapes_h
#define shapes_h

#include "common.h"
#include "material.h"
#include "ray.h"

typedef enum ShapeType {
    Sphere,
    Plane,
} ShapeType;

typedef struct Shape {
    float4x4 transform_inverse;
    ShapeType shape;
    uint32_t index;
    Material material;
    
    float4 normal_at(float4 world_pos) const;
    void intersect(const thread Ray& world_ray, thread Intersections& hits) const;
    void local_intersect_sphere(const thread Ray& object_ray, thread Intersections& hits) const;
    void local_intersect_plane(const thread Ray& object_ray, thread Intersections& hits) const;
} Shape;

#endif
