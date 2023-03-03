#ifndef RAYTRACER_PLANE_H
#define RAYTRACER_PLANE_H

#include "Shape.h"
class Plane : public Shape {
public:
    Tuple local_normal_at(const Tuple& object_space_point) const override;
    void local_intersect(const Ray& object_space_ray, Intersections& locations) const override;
    Shape* copy() const override;

    // For tests that don't use Shape pointers so don't see abstract methods.
    Intersections local_intersect(const Ray &object_space_ray) const {
        return Shape::intersect(object_space_ray);
    }
};

#endif
