#ifndef ray_h
#define ray_h

#include "common.h"

typedef struct Ray {
    float4 origin;
    float4 direction;
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

#endif
