#ifndef RAYTRACER_PLANE_H
#define RAYTRACER_PLANE_H

#include "Shape.h"
class Plane : public Shape {
public:
    Tuple local_normal_at(const Tuple& object_space_point) const override;
    Intersections local_intersect(const Ray& object_space_ray) const override;
    Shape* copy() const override;
};

#endif
