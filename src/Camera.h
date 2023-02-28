#ifndef RAYTRACER_CAMERA_H
#define RAYTRACER_CAMERA_H

#include "Matrix.h"
#include "World.h"
#include "Canvas.h"

class Camera {
public:
    MemoMatrix transform;
    int hsize;
    int vsize;
    double field_of_view;  // radians!
    double pixel_size;
    double half_width;
    double half_height;

    Camera(int hsize, int vsize, double field_of_view);
    void set_transform(const Matrix& m){
        transform = MemoMatrix(m);
    }

    Ray ray_for_pixel(int x, int y) const;
    Canvas render(const World& world) const;
};


#endif
