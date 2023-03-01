#include "Sphere.h"

Tuple Sphere::local_normal_at(const Tuple& object_space_point) const {
    return object_space_point.subtract(Point(0, 0, 0));
}

Intersections Sphere::local_intersect(const Ray& ray) const {
    Tuple sphere_to_ray = ray.origin.subtract(Point(0, 0, 0));
    double a = ray.direction.dot(ray.direction);
    double b = 2 * ray.direction.dot(sphere_to_ray);
    double c = sphere_to_ray.dot(sphere_to_ray) - 1;
    double discriminant = (b * b) - (4 * a * c);

    Intersections locations;

    if (discriminant >= 0) {
        double t1 = (-b - sqrt(discriminant)) / (2 * a);
        double t2 = (-b + sqrt(discriminant)) / (2 * a);

        locations.add(Intersection(t1, *(Shape *) this));
        locations.add(Intersection(t2, *(Shape *) this));
    }

    return locations;
}

Shape* Sphere::copy() const {
    Sphere* shape = new Sphere();
    shape->material = material;
    shape->set_transform(transform);
    return shape;
}
