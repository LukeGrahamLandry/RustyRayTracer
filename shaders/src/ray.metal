#include "ray.h"

Ray Camera::ray_for_pixel(float x, float y) const constant {
    // We want the middle of the pixel.
    // Adjusted from canvas space to world space units.
    // Since the camera is at (0, 0), translate. This flips it so high y becomes negative.
    // But canvas units are kinda flipped too, so it cancels out? And canvas looks at -x so x flip works too.
    float object_x = half_width - (((x + 0.5)) * pixel_size);
    float object_y = half_height - (((y + 0.5)) * pixel_size);

    // Position of the pixel in the camera's object space.
    float4 pixel_object_point = Point(object_x, object_y, -1);

    // Transform to world space.
    float4 pixel_world_point = transform_inverse * pixel_object_point;
    float4 camera_world_point = transform_inverse * Point(0, 0, 0);
    float4 ray_direction = pixel_world_point - camera_world_point;
    return {camera_world_point, ray_direction};
};
