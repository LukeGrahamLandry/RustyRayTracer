#include "Sphere.h"

Sphere::Sphere() {
    set_transform(Transformation::identity());
}

Intersections Sphere::intersect(const Ray& world_space_ray) {
    Ray ray = world_space_ray.transform(transform.inverse());

    Tuple sphere_to_ray = ray.origin.subtract(Point(0, 0, 0));
    float a = ray.direction.dot(ray.direction);
    float b = 2 * ray.direction.dot(sphere_to_ray);
    float c = sphere_to_ray.dot(sphere_to_ray) - 1;
    float discriminant = (b * b) - (4 * a * c);

    Intersections locations;

    if (discriminant >= 0) {
        float t1 = (-b - sqrt(discriminant)) / (2 * a);
        float t2 = (-b + sqrt(discriminant)) / (2 * a);

        locations.add(Intersection(t1, *this));
        locations.add(Intersection(t2, *this));
    }

    return locations;
}

bool Sphere::equals(const Sphere& sphere) const {
    return transform.equals(sphere.transform);
}

void Sphere::set_transform(const Matrix& m) {
    transform = MemoMatrix(m);
}

Tuple Sphere::normal_at(const Tuple& world_space_point) const {
    Tuple object_space_point = transform.inverse().multiply(world_space_point);
    Tuple object_space_normal = object_space_point.subtract(Point(0, 0, 0));
    Tuple world_space_normal =  transform.transpose_of_inverse().multiply(object_space_normal);
    world_space_normal.set(3, 0);  // cringe

    return world_space_normal.normalize();
}
