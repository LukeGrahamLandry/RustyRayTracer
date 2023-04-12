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
    
    inline bool operator==(const thread Intersection& rhs) const {
        return t == rhs.t && obj == rhs.obj;
    }
} Intersection;

typedef struct Intersections {
    int count;
    bool is_hit;
    Intersection hits[MAX_HITS];
    
    Intersections(){
        clear();
    }
    Intersection get_hit() const;
    void add(float t, int shape_index);
    inline bool has_hit() const {
        return is_hit;
    };
    inline void clear() {
        count = 0;
        is_hit = false;
    }
    inline bool is_empty() const {
        return count == 0;
    }
    inline const thread Intersection& last() const {
        return hits[count - 1];
    }
    int index_of(const thread Intersection& hit) const;
    void remove(int i);
} Intersections;

typedef struct RayInfo {
    Ray ray;
    float weight;
} RayInfo;

typedef struct RayQueue {
    RayInfo rays[MAX_RAY_QUEUE];
    int start;
    int end;
    
    RayQueue(){
        start = 0;
        end = 0;
    }
    
    inline RayInfo pop() {
        int index = start % MAX_RAY_QUEUE;
        start++;
        return rays[index];
    }
    
    inline void push(const thread Ray& r, float weight) {
        int count = end - start;
        if (count > MAX_RAY_QUEUE) return;
        int index = end % MAX_RAY_QUEUE;
        rays[index] = {r, weight};
        end++;
    }
    
    inline bool is_empty() const {
        return start == end;
    }
} RayQueue;


#include "shapes.h"

#endif
