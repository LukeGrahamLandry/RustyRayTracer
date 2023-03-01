#ifndef RAYTRACER_SPHERE_H
#define RAYTRACER_SPHERE_H

#include "Shape.h"
class Sphere : public Shape {
public:
    Tuple local_normal_at(const Tuple& object_space_point) const override;
    Intersections local_intersect(const Ray& object_space_ray) const override;
};

#endif
