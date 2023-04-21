#include "shapes.h"
#include "common.h"

float4 Shape::normal_at(float4 world_pos) const {
    float4 object_space_point = transform_inverse * world_pos;
    float4 object_space_normal;
    
    switch (shape) {
        case Sphere: {
            object_space_normal = object_space_point - point(0, 0, 0);
            break;
        }
        case Plane: {
            object_space_normal = vector(0.0, 1.0, 0.0);
            break;
        }
        case Cube: {
            float maxc = max3(abs(object_space_point.x), abs(object_space_point.y), abs(object_space_point.z));
            if (maxc == abs(object_space_point.x)) {
                object_space_normal = vector(object_space_point.x, 0, 0);
            } else if (maxc == abs(object_space_point.y)) {
                object_space_normal = vector(0, object_space_point.y, 0);
            } else {
                object_space_normal = vector(0, 0, object_space_point.z);
            }
            break;
        }
    }
    
    float4 world_space_normal = transpose(transform_inverse) * object_space_normal;
    world_space_normal.w = 0;
    return normalize(world_space_normal);
}

void Shape::intersect(const thread Ray& world_ray, thread Intersections& hits) const {
    Ray object_space_ray = world_ray.transform(transform_inverse);
    // Look at me. Look at me. I am the VTable now.
    switch (shape) {
        case Sphere: {
            return local_intersect_sphere(object_space_ray, hits);
        }
        case Plane: {
            return local_intersect_plane(object_space_ray, hits);
        }
        case Cube: {
            return local_intersect_cube(object_space_ray, hits);
        }
    }
}

void Shape::local_intersect_sphere(const thread Ray& ray, thread Intersections& hits) const {
    float4 sphere_to_ray = ray.origin - point(0, 0, 0);
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

float2 check_axis(float origin, float direction){
    float tmin_numerator = (-1 - origin);
    float tmax_numerator = (1 - origin);
    float tmin, tmax;
    if (abs(direction) >= EPSILON){
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {  // I think metal fast-math assumes no infinity
        tmin = tmin_numerator * 999999999.0f;
        tmax = tmax_numerator * 999999999.0f;
    }

    if (tmin > tmax)  {
        return float2(tmax, tmin);
    } else {
        return float2(tmin, tmax);
    }
}


void Shape::local_intersect_cube(const thread Ray& ray, thread Intersections& hits) const {
    float2 x = check_axis(ray.origin.x, ray.direction.x);
    float2 y = check_axis(ray.origin.y, ray.direction.y);
    float2 z = check_axis(ray.origin.z, ray.direction.z);

    float tmin = max3(x.x, y.x, z.x);
    float tmax = min3(x.y, y.y, z.y);

    if (tmin <= tmax){
        hits.add(tmin, index);
        hits.add(tmax, index);
    }
}
