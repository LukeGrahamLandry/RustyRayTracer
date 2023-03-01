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
    Tuple position(double t) const;
    Ray transform(const Matrix& transformation) const;
};

class Shape;

struct IntersectionComps {
    double t;
    Shape* object;
    Tuple point;
    Tuple eyev;
    Tuple normalv;
    bool inside;
    Tuple over_point;
};

class Intersection {
public:
    Intersection();
    Intersection(const Intersection &other);

    double t;
    Shape* object;

    Intersection(double tIn, Shape& objectIn);

    IntersectionComps prepare_computations(const Ray& ray) const;

    bool equals(const Intersection& other) const;
};

class Intersections {
public:
    vector<Intersection*> intersections;

    Intersections();
    ~Intersections();
    Intersections(initializer_list<Intersection> group);

    void add(const Intersection& intersection);

    int count() const;

    Intersection get(int index);
    Intersection hit();
    bool hasHit();

    void addAll(const Intersections& intersections);
};

#include "shapes/Shape.h"

#endif
