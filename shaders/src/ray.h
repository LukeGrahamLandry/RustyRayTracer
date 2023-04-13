#ifndef ray_h
#define ray_h

#include "common.h"

struct Shape;

typedef struct Ray {
    float4 origin;
    float4 direction;
    
    Ray() : Ray(float4(0), float4(0)) {};
    Ray(float4 o, float4 d) : origin(o), direction(d) {};
    Ray transform(float4x4 mat) const;
    float4 position(float t) const {
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
    
    bool operator==(const thread Intersection& rhs) const {
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
    bool has_hit() const {
        return is_hit;
    };
    void clear() {
        count = 0;
        is_hit = false;
    }
    bool is_empty() const {
        return count == 0;
    }
    const thread Intersection& last() const {
        return hits[count - 1];
    }
    int index_of(const thread Intersection& hit) const;
    void remove(int i);
} Intersections;

typedef struct RayInfo {
    Ray ray;
    float weight;
    
    RayInfo(Ray r, float w) : ray(r), weight(w) {};
    RayInfo() : ray(Ray()), weight(0) {};
} RayInfo;

typedef struct RayQueue {
    RayInfo rays[MAX_RAY_QUEUE];
    int start;
    int end;
    
    RayQueue(){
        start = 0;
        end = 0;
    }
    
    RayInfo pop() {
        int index = start % MAX_RAY_QUEUE;
        start++;
        return rays[index];
    }
    
    void push(Ray r, float weight) {
        // TODO: bounds checking seems very cringe but also i dont want it to stomp on stuff.
        //       currently drops later ones instead of earlier ones which seems better but maybe not worth it.
        int count = end - start;
        if (count > MAX_RAY_QUEUE) return;
        int index = end % MAX_RAY_QUEUE;
        rays[index] = RayInfo(r, weight);
        end++;
    }
    
    bool is_empty() const {
        return start == end;
    }
} RayQueue;


#include "shapes.h"

#endif
