#include "world.h"

// Since Metal doesn't allow recursion in fragment shaders, this iteratively processes a queue of rays.
// When a new ray needs to be spawned for a reflection or refraction, it just gets pushed to the queue.
float3 WorldView::colour_at(const thread Ray& first_ray) const {
    float3 colour = BLACK();
    Intersections hits;
    RayQueue queue;
    queue.push(first_ray, 1.0);
    for (int i=0;i<MAX_REFLECT_REFRACT && !queue.is_empty();i++) {
        RayInfo ray = queue.pop();
        intersect(ray.ray, hits);

        if (hits.has_hit()) {
            Comps comps = prepare_comps(hits.get_hit(), ray.ray, hits);
            colour += shade_hit(comps) * ray.weight;

            float reflect_weight = ray.weight * comps.material.reflective;
            if (reflect_weight > EPSILON) {
                queue.push(Ray {comps.over_point, comps.reflectv}, reflect_weight);
            }
            
            // https://en.wikipedia.org/wiki/Snell%27s_law
            float refract_weight = ray.weight * comps.material.transparency;
            if (refract_weight > EPSILON){
                float n_ratio = comps.n1 / comps.n2;
                float cos_i = dot(comps.eyev, comps.normalv);
                float sin2_t = n_ratio*n_ratio * (1 - cos_i*cos_i);
                
                if (sin2_t < 1){  // not total internal reflection
                    float cos_t = sqrt(1 - sin2_t);
                    float4 direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
                    queue.push(Ray {comps.under_point, direction}, refract_weight);
                }
            }
            
            hits.clear();
        }
    }
    
    return colour;
}

void WorldView::intersect(const thread Ray& ray, thread Intersections& hits) const {
    for (uint32_t i=0;i<inputs.shape_count;i++){
        Shape shape = shapes[i];
        shape.intersect(ray, hits);
    }
}

float3 WorldView::shade_hit(const thread Comps& comps) const {
    float3 colour = BLACK();
    for (uint32_t i=0;i<inputs.light_count;i++){
        PointLight light = lights[i];
        bool shadowed = is_shadowed(light.position, comps.over_point);
        colour += comps.material.lighting(light, comps.over_point, comps.eyev, comps.normalv, shadowed);
    }
    
    return colour;
}

// TODO: just check for a hit in the range without sorting the whole Intersections 
bool WorldView::is_shadowed(const thread float4& light_pos, const thread float4& hit_pos) const {
    float4 light_direction = light_pos - hit_pos;
    Ray ray = {hit_pos, normalize(light_direction)};
    Intersections hits;
    intersect(ray, hits);
    if (hits.has_hit()) {
        float t = hits.get_hit().t;
        // Checks that the hit is not behind the light.
        return t*t < length_squared(light_direction);
    } else {
        return false;
    }
}

Comps WorldView::prepare_comps(const thread Intersection& hit, const thread Ray& ray, const thread Intersections& xs) const {
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
    refraction_path(comps, hit, xs);
    return comps;
}

// TODO: really feels like this shouldn't need to use an extra list.
void WorldView::refraction_path(thread Comps& comps, const thread Intersection& hit, const thread Intersections& xs) const {
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
}
