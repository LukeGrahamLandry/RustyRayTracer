#ifndef RAYTRACER_RAY_H
#define RAYTRACER_RAY_H

#include <vector>
#include "Tuple.h"
#include "Matrix.h"

class Ray {
public:
    Tuple origin;
    Tuple direction;

    Ray();
    Ray(const Tuple& origin, const Tuple& direction);
    Tuple position(float t) const;
    Ray transform(const Matrix& transformation) const;
};

class Sphere;

class Intersection {
public:
    Intersection();

    float t;
    Sphere* object;
    Intersection(float tIn, Sphere& objectIn);

    bool equals(const Intersection& other) const;
};

class Intersections {
public:
    vector<Intersection> intersections;

    Intersections();
    Intersections(initializer_list<Intersection> group);

    void add(Intersection intersection);

    int count();

    Intersection get(int index);
    Intersection hit();
    bool hasHit();
};

#include "shapes/Sphere.h"

#endif
