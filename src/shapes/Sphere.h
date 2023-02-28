#ifndef RAYTRACER_SPHERE_H
#define RAYTRACER_SPHERE_H

#include "Matrix.h"
#include "Material.h"

class Ray;
class Intersection;
class Intersections;

class Sphere {
public:
    Matrix transform;
    Matrix inverse_transform;
    Matrix transpose_inverse_transform;
    Material material;
    Sphere();
    Intersections intersect(const Ray& ray);
    void set_transform(Matrix m);
    Tuple normal_at(const Tuple& point) const;

    bool equals(const Sphere& sphere) const;
};


#include "Ray.h"

#endif
