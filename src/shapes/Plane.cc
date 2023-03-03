#include "Plane.h"

Tuple Plane::local_normal_at(const Tuple& object_space_point) const {
    return Vector(0, 1, 0);
}

void Plane::local_intersect(const Ray& ray, Intersections& locations) const {
    if (!almostEqual(ray.direction.y(), 0)){
        double t = -ray.origin.y() / ray.direction.y();
        locations.add(Intersection(t, *(Shape *) this));
    }
}

Shape* Plane::copy() const {
    Plane* shape = new Plane();
    shape->material = material;
    shape->set_transform(transform);
    return shape;
}
