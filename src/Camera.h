#ifndef RAYTRACER_CAMERA_H
#define RAYTRACER_CAMERA_H

#include "Matrix.h"
#include "World.h"
#include "Canvas.h"
#include "common.h"
#include <thread>

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
    const bool do_progress_logging;
};


class RenderTask {
public:
    const World& world;
    const Camera& camera;
    long start_time;
    long end_time;
    bool active;
    bool killed;
    int thread_count;
    thread* threads;
    int finished_count;
    Canvas* canvas;
    int frameIndex;

    RenderTask(const World &world, const Camera &camera);
    ~RenderTask();

    void start();
    void halt();
    void renderSlice(int xStart, int xEnd);
    void setResolution(int x, int y);
    void waitForEnd();
    void setThreadCount(int x);

    const Canvas& getCanvas() const {
        return *canvas;
    }

    bool isDone() const {
        return finished_count == camera.hsize;
    }
};


#endif
