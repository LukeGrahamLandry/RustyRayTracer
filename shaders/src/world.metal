#include "world.h"


float3 World::color_at(Ray ray) const {
    float3 colour = float3(0);
    float prev_reflectance = 1;
    for (int i=0;i<MAX_REFLECTIONS;i++) {
        Intersections hits = intersections();
        intersect(ray, hits);

        if (hits.has_hit()) {
            Comps comps = prepare_comps(hits.get_hit(), ray);
            colour += shade_hit(comps) * prev_reflectance;

            prev_reflectance *= comps.material.reflective;
            if (prev_reflectance < EPSILON) {
                break;
            }
            ray = Ray {comps.over_point, comps.reflectv};
        } else {
            break;
        }
    }
    
    return colour;
}

void World::intersect(const thread Ray& ray, thread Intersections& hits) const {
    for (uint32_t i=0;i<inputs.shape_count;i++){
        Shape shape = shapes[i];
        shape.intersect(ray, hits);
    }
}

float3 World::shade_hit(const thread Comps& comps) const {
    float3 colour = float3(0);
    for (uint32_t i=0;i<inputs.light_count;i++){
        PointLight light = lights[i];
        colour += comps.material.lighting(light, comps.over_point, comps.eyev, comps.normalv, is_shadowed(light.position, comps.over_point));
    }
    
    return colour;
}

bool World::is_shadowed(const thread float4& light_pos, const thread float4& hit_pos) const {
    float4 light_direction = light_pos - hit_pos;
    Ray ray = {hit_pos, normalize(light_direction)};
    Intersections hits = intersections();
    intersect(ray, hits);
    // Make sure the hit is not behind the light.
    if (hits.has_hit()) {
        float t = hits.get_hit().t;
        return t*t < length_squared(light_direction);
    } else {
        return false;
    }
}

Comps World::prepare_comps(const thread Intersection& hit, const thread Ray& ray) const {
    Shape object = shapes[hit.obj];
    Comps comps;
    comps.t = hit.t;
    comps.material = object.material;
    comps.point = ray.position(hit.t);
    comps.eyev = -ray.direction;
    comps.normalv = object.normal_at(comps.point);
    comps.inside = dot(comps.normalv, comps.eyev) < 0;
    if (comps.inside) comps.normalv = -comps.normalv;

    // Used for is_shadowed checks to prevent shadow acne
    comps.over_point = comps.point + (comps.normalv * EPSILON);
    comps.reflectv = reflect(ray.direction, comps.normalv);
    return comps;
}
