#include "shapes.h"

float4 Shape::normal_at(float4 world_pos) const {
    float4 object_space_point = transform_inverse * world_pos;
    float4 object_space_normal;
    
    switch (shape) {
        case Sphere: {
            object_space_normal = object_space_point - Point(0, 0, 0);
            break;
        }
        case Plane: {
            object_space_normal = Vector(0.0, 1.0, 0.0);
            break;
        }
        default: {
            __builtin_unreachable();
        }
    }
    
    float4 world_space_normal = transpose(transform_inverse) * object_space_normal;
    world_space_normal.w = 0;
    return normalize(world_space_normal);
}

void Shape::intersect(const thread Ray& world_ray, thread Intersections& hits) const {
    Ray object_space_ray = world_ray.transform(transform_inverse);
    switch (shape) {
        case Sphere: {
            return local_intersect_sphere(object_space_ray, hits);
        }
        case Plane: {
            return local_intersect_plane(object_space_ray, hits);
        }
    }
    
    __builtin_unreachable();
}

void Shape::local_intersect_sphere(const thread Ray& ray, thread Intersections& hits) const {
    float4 sphere_to_ray = ray.origin - Point(0, 0, 0);
    float a = dot(ray.direction, ray.direction);
    float b = 2 * dot(ray.direction, sphere_to_ray);
    float c = dot(sphere_to_ray, sphere_to_ray) - 1;
    float discriminant = (b * b) - (4 * a * c);

    if (discriminant >= 0) {
        float d = sqrt(discriminant);
        float t1 = (-b - d) / (2 * a);
        float t2 = (-b + d) / (2 * a);

        hits.add(t1, index);
        hits.add(t2, index);
    }
}

void Shape::local_intersect_plane(const thread Ray& ray, thread Intersections& hits) const {
    if (abs(ray.direction.y) > 0) {
        float t = -ray.origin.y / ray.direction.y;
        hits.add(t, index);
    }
}
