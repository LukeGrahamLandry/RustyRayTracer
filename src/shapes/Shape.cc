#include "Shape.h"

Intersections Shape::intersect(const Ray& world_space_ray) const {
    Ray ray = world_space_ray.transform(transform.inverse());
    return local_intersect(ray);
}

void Shape::intersect(const Ray& world_space_ray, Intersections& locations) const {
    Ray ray = world_space_ray.transform(transform.inverse());
    local_intersect(ray, locations);
}


bool Shape::equals(const Shape& Shape) const {
    return transform.equals(Shape.transform);
}

void Shape::set_transform(const Matrix& m) {
    transform = MemoMatrix(m);
}

Tuple Shape::normal_at(const Tuple& world_space_point) const {
    Tuple object_space_point = transform.inverse().multiply(world_space_point);
    Tuple object_space_normal = local_normal_at(object_space_point);
    Tuple world_space_normal =  transform.transpose_of_inverse().multiply(object_space_normal);
    world_space_normal.set(3, 0);  // cringe
    return world_space_normal.normalize();
}

Shape::Shape() {
    set_transform(Transformation::identity());
}

Intersections Shape::local_intersect(const Ray &object_space_ray) const {
    Intersections locations;
    local_intersect(object_space_ray, locations);
    return locations;
}