#ifndef ray_h
#define ray_h

#include "common.h"

struct Shape;

typedef struct Ray {
    float4 origin;
    float4 direction;
    
    Ray transform(float4x4 mat) const;
    inline float4 position(float t) const {
        return origin + (direction * t);
    }
} Ray;

typedef struct Camera {
    float4x4 transform_inverse;
    float pixel_size;
    float half_width;
    float half_height;
    float hsize;
    float vsize;
    
    Ray ray_for_pixel(float x, float y) const constant;
} Camera;

typedef struct Intersection {
    float t;
    int obj;
} Intersection;

typedef struct Intersections {
    int count;
    bool is_hit;
    Intersection hits[MAX_HITS];
    
    Intersection get_hit() const;
    void add(float t, int shape_index);
    inline bool has_hit() const {
        return is_hit;
    };
    inline void clear() {
        count = 0;
        is_hit = false;
    }
} Intersections;

// TODO: default constructor 
inline Intersections intersections() {
    return {0, false, {}};
}

#include "shapes.h"

#endif
