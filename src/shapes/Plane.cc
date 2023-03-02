#include "Plane.h"

Tuple Plane::local_normal_at(const Tuple& object_space_point) const {
    return Vector(0, 1, 0);
}

Intersections Plane::local_intersect(const Ray& ray) const {
    Intersections locations;
    if (!almostEqual(ray.direction.y(), 0)){
        double t = -ray.origin.y() / ray.direction.y();
        locations.add(Intersection(t, *(Shape *) this));
    }
    return locations;
}

Shape* Plane::copy() const {
    Plane* shape = new Plane();
    shape->material = material;
    shape->set_transform(transform);
    return shape;
}
