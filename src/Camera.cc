#include "Camera.h"

Camera::Camera(int hsize, int vsize, float field_of_view) : hsize{hsize}, vsize{vsize}, field_of_view{field_of_view} {
    set_transform(Transformation::identity());
    float half_view = tan(field_of_view / 2);
    float aspect_ratio = ((float) hsize) / ((float) vsize);
    half_width = aspect_ratio >= 1 ? half_view : half_view * aspect_ratio;
    half_height = aspect_ratio >= 1 ? half_view / aspect_ratio : half_view;
    pixel_size = (half_width * 2) / ((float) hsize);
}

// x and y are in canvas space.
Ray Camera::ray_for_pixel(int x, int y) const {
    // We want the middle of the pixel.
    // Adjusted from canvas space to world space units.
    // Since the camera is at (0, 0), translate. This flips it so high y becomes negative.
    // But canvas units are kinda flipped too, so it cancels out? And canvas looks at -x so x flip works too.
    float object_x = half_width - (((float) (x + 0.5)) * pixel_size);
    float object_y = half_height - (((float) (y + 0.5)) * pixel_size);

    // Position of the pixel in the camera's object space.
    Tuple pixel_object_point = Point(object_x, object_y, -1);

    // Transform to world space.
    Tuple pixel_world_point = transform.inverse().multiply(pixel_object_point);
    Tuple camera_world_point = transform.inverse().multiply(Point(0, 0, 0));
    Tuple ray_direction = pixel_world_point.subtract(camera_world_point);
    return Ray(camera_world_point, ray_direction);
}

Canvas Camera::render(const World& world) const {
    int width = (int) (half_width * 2 * pixel_size);
    int height = (int) (half_height * 2 * pixel_size);
    Canvas canvas = Canvas(hsize, vsize);

    for (int x=0;x<hsize;x++){
        for (int y=0;y<vsize;y++){
            Ray ray = ray_for_pixel(x, y);
            Colour colour = world.color_at(ray);
            canvas.write_pixel(x, y, colour);
        }
    }

    return canvas;
}
