#include "world.h"


float3 World::color_at(const thread Ray& first_ray) const {
    float3 colour = float3(0);
    Intersections hits;
    RayQueue queue;
    queue.push(first_ray, 1.0);
    for (int i=0;i<MAX_REFLECTIONS && !queue.is_empty();i++) {
        RayInfo ray = queue.pop();
        intersect(ray.ray, hits);

        if (hits.has_hit()) {
            Comps comps = prepare_comps(hits.get_hit(), ray.ray, hits);
            colour += shade_hit(comps) * ray.weight;

            float reflect_weight = ray.weight * comps.material.reflective;
            if (reflect_weight > EPSILON) {
                queue.push(Ray {comps.over_point, comps.reflectv}, reflect_weight);
            }
            hits.clear();
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
        bool shadowed = is_shadowed(light.position, comps.over_point);
        colour += comps.material.lighting(light, comps.over_point, comps.eyev, comps.normalv, shadowed);
    }
    
    return colour;
}

bool World::is_shadowed(const thread float4& light_pos, const thread float4& hit_pos) const {
    float4 light_direction = light_pos - hit_pos;
    Ray ray = {hit_pos, normalize(light_direction)};
    Intersections hits;
    intersect(ray, hits);
    // Make sure the hit is not behind the light.
    if (hits.has_hit()) {
        float t = hits.get_hit().t;
        return t*t < length_squared(light_direction);
    } else {
        return false;
    }
}

Comps World::prepare_comps(const thread Intersection& hit, const thread Ray& ray, const thread Intersections& xs) const {
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
    comps.under_point = comps.point - (comps.normalv * EPSILON);
    comps.reflectv = reflect(ray.direction, comps.normalv);
    
    Intersections containers;
    for (int i=0;i<xs.count;i++){
        Intersection check = xs.hits[i];
        if (hit == check){
            if (containers.is_empty()) {
                comps.n1 = 1.0;
            } else {
                Shape s = shapes[xs.last().obj];
                comps.n1 = s.material.refractive_index;
            }
        }
        
        int index = containers.index_of(check);
        if (index >= 0){
            containers.remove(index);
        } else {
            containers.add(check.t, check.obj);
        }
        
        if (hit == check){
            if (containers.is_empty()) {
                comps.n2 = 1.0;
            } else {
                Shape s = shapes[xs.last().obj];
                comps.n2 = s.material.refractive_index;
            }
            break;
        }
    }
    
    return comps;
}
