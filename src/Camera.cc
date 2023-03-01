#include "Camera.h"

Camera::Camera(int hsize, int vsize, double field_of_view, bool do_progress_logging) : hsize{hsize}, vsize{vsize}, field_of_view{field_of_view}, do_progress_logging{do_progress_logging} {
    set_transform(Transformation::identity());
    double half_view = tan(field_of_view / 2);
    double aspect_ratio = ((double) hsize) / ((double) vsize);
    half_width = aspect_ratio >= 1 ? half_view : half_view * aspect_ratio;
    half_height = aspect_ratio >= 1 ? half_view / aspect_ratio : half_view;
    pixel_size = (half_width * 2) / ((double) hsize);
}

// x and y are in canvas space.
Ray Camera::ray_for_pixel(int x, int y) const {
    // We want the middle of the pixel.
    // Adjusted from canvas space to world space units.
    // Since the camera is at (0, 0), translate. This flips it so high y becomes negative.
    // But canvas units are kinda flipped too, so it cancels out? And canvas looks at -x so x flip works too.
    double object_x = half_width - (((double) (x + 0.5)) * pixel_size);
    double object_y = half_height - (((double) (y + 0.5)) * pixel_size);

    // Position of the pixel in the camera's object space.
    Tuple pixel_object_point = Point(object_x, object_y, -1);

    // Transform to world space.
    Tuple pixel_world_point = transform.inverse().multiply(pixel_object_point);
    Tuple camera_world_point = transform.inverse().multiply(Point(0, 0, 0));
    Tuple ray_direction = pixel_world_point.subtract(camera_world_point);
    return Ray(camera_world_point, ray_direction);
}

Canvas Camera::render(const World& world) const {
    long start_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();
    Canvas canvas = Canvas(hsize, vsize);

    for (int x=0;x<hsize;x++){
        for (int y=0;y<vsize;y++){
            Ray ray = ray_for_pixel(x, y);
            Colour colour = world.color_at(ray);
            canvas.write_pixel(x, y, colour);
        }

        if (do_progress_logging){
            cout << x << "/" << hsize << endl;
        }
    }

    long end_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();
    if (do_progress_logging) {
        cout << "Rendered " << (hsize * vsize) << " pixels in " << (end_time - start_time) << " ms." << endl;
    }

    return canvas;
}
