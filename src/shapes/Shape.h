#ifndef RAYTRACER_SHAPE_H
#define RAYTRACER_SHAPE_H

#include "Matrix.h"
#include "Material.h"

class Ray;
class Intersection;
class Intersections;

class Shape {
public:
    MemoMatrix transform;
    Material material;
    Intersections intersect(const Ray& ray) const;
    void set_transform(const Matrix& m);
    Tuple normal_at(const Tuple& point) const;

    Shape();
    virtual ~Shape() = default;
    virtual Shape* copy() const = 0;
    virtual Tuple local_normal_at(const Tuple& object_space_point) const = 0;
    virtual Intersections local_intersect(const Ray& object_space_ray) const = 0;

    bool equals(const Shape& shape) const;
};

#include "Ray.h"


#endif
