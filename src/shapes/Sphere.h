#ifndef RAYTRACER_SPHERE_H
#define RAYTRACER_SPHERE_H

#include "Matrix.h"

class Ray;
class Intersection;
class Intersections;

class Sphere {
public:
    Matrix transform;
    Sphere();
    Intersections intersect(const Ray& ray);
    void set_transform(Matrix m);

    bool equals(const Sphere& sphere) const;
};


#include "Ray.h"

#endif
