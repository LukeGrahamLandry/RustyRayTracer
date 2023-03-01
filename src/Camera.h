#ifndef RAYTRACER_CAMERA_H
#define RAYTRACER_CAMERA_H

#include "Matrix.h"
#include "World.h"
#include "Canvas.h"
#include "common.h"
#include <thread>

struct RenderState {
    int count;
    long next_log_time;
    long start_time;
    long end_time;
    int log_interval_ms;
    function<void(int x)> callback;

    RenderState(int log_interval_ms, const function<void(int x)>& callback){
        next_log_time = -1;
        start_time = -1;
        end_time = -1;
        count = 0;
        this->callback = callback;
        this->log_interval_ms = log_interval_ms;
    }
};

class Camera {
public:
    MemoMatrix transform;
    int hsize;
    int vsize;
    double field_of_view;  // radians!
    double pixel_size;
    double half_width;
    double half_height;

    Camera(int hsize, int vsize, double field_of_view, bool do_progress_logging);
    Camera(int hsize, int vsize, double field_of_view): Camera(hsize, vsize, field_of_view, false){

    };
    void set_transform(const Matrix& m){
        transform = MemoMatrix(m);
    }

    Ray ray_for_pixel(int x, int y) const;
    Canvas render(const World& world) const;
    vector<thread> startRender(Canvas& canvas, const World& world, RenderState& progress) const;

    void renderSlice(const World& world, Canvas& canvas, int xStart, int xEnd, RenderState& progress) const;

private:
    const bool do_progress_logging;
    static int log_interval_ms;
};


#endif
