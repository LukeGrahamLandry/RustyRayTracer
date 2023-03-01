#include "Camera.h"

int Camera::log_interval_ms = 300;

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
    Canvas canvas = Canvas(hsize, vsize);

    RenderState progress = RenderState(log_interval_ms, [&](int unused) -> void {
        if (do_progress_logging) cout << progress.count << "/" << hsize << endl;
    });

    vector<thread> threads = startRender(canvas, world, progress);
    for (thread& t : threads){
        t.join();
    }

    long end_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();
    if (do_progress_logging) {
        cout << "Rendered " << (hsize * vsize) << " pixels in " << (end_time - progress.start_time) << " ms." << endl;
    }

    return canvas;
}

vector<thread> Camera::startRender(Canvas& canvas, const World &world, RenderState& progress) const {
    int thread_count = max((int) thread::hardware_concurrency(), 1);
    int slice_width = hsize / thread_count;
    int extra = hsize - (slice_width * thread_count);

    progress.start_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();
    progress.next_log_time = progress.start_time + progress.log_interval_ms;
    if (do_progress_logging) cout << "Rendering " << (hsize * vsize) << " pixels with " << thread_count << " threads" << endl;

    // TODO: don't have continuous blocks so its less likely that the edge ones get mostly black
    vector<thread> threads;
    for (int t=0;t<thread_count;t++){
        threads.emplace_back(&Camera::renderSlice, this, ref(world), ref(canvas), t * slice_width, (t + 1) * slice_width, ref(progress));
    }
    threads.emplace_back(&Camera::renderSlice, this, ref(world), ref(canvas), hsize - extra, hsize, ref(progress));

    return threads;
}


void Camera::renderSlice(const World &world, Canvas &canvas, int xStart, int xEnd, RenderState& progress) const {
    int step = 3;

    for (int lx=0;lx<step;lx++){
        for (int ly=0;ly<step;ly++){
            for (int x=xStart+lx;x<xEnd;x+=step){
                for (int y=ly;y<vsize;y+=step){
                    Ray ray = ray_for_pixel(x, y);
                    Colour colour = world.color_at(ray);
                    canvas.write_pixel(x, y, colour);
                }
            }
        }
    }

    progress.count += xEnd - xStart;
    progress.callback(0);
}
