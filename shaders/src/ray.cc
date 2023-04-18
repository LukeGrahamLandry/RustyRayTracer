#include "ray.h"

Ray Camera::ray_for_pixel(float x, float y) const constant {
    // We want the middle of the pixel.
    // Adjusted from canvas space to world space units.
    // Since the camera is at (0, 0), translate. This flips it so high y becomes negative.
    // But canvas units are kinda flipped too, so it cancels out? And canvas looks at -x so x flip works too.
    float object_x = half_width - (((x + 0.5f)) * pixel_size);
    float object_y = half_height - (((y + 0.5f)) * pixel_size);

    // Position of the pixel in the camera's object space.
    float4 pixel_object_point = point(object_x, object_y, -1);

    // Transform to world space.
    float4 pixel_world_point = transform_inverse * pixel_object_point;
    float4 camera_world_point = transform_inverse * point(0, 0, 0);
    float4 ray_direction = normalize(pixel_world_point - camera_world_point);
    return Ray(camera_world_point, ray_direction);
};


Ray Ray::transform(float4x4 mat) const {
    return Ray(mat * origin, mat * direction);
}

float4 Ray::position(float t) const {
    return origin + (direction * t);
}

// TODO: better to just sort once at the end? do CSG then decide
//       seems like you mostly don't need the whole thing sorted
//       except for refraction so doing it for shadows too is strange,
//       should just look for one in the right range without swapping.
void Intersections::add(float t, uint32_t shape_index) {
    Intersection hit = {t, shape_index};
    if (hit.t >= 0) {
        is_hit = true;
    }

    for (int i=0;i<count;i++) {
        if (hit.t < hits[i].t) {
            Intersection temp = hits[i];
            hits[i] = hit;
            hit = temp;
        }
    }

    hits[count] = hit;
    count += 1;
}


Intersection Intersections::get_hit() const {
    for (int i=0; i<count; i++) {
        if (hits[i].t >= 0) {
            return hits[i];
        }
    }
    
    // check has_hit first
    __builtin_unreachable();
};


int Intersections::index_of(const thread Intersection& hit) const {
    for (int i=0;i<count;i++) {
        if (hits[i] == hit) return i;
    }
    return -1;
}


void Intersections::remove(int index) {
    for (int i=index;i<(count-1);i++){
        hits[i] = hits[i+1];
    }
    count--;
}
