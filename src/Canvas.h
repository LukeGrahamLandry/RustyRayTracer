#ifndef RAYTRACER_CANVAS_H
#define RAYTRACER_CANVAS_H

#include "Colour.h"

class Canvas {
public:
    Canvas();
    Canvas(int width, int height);
    ~Canvas();
    Canvas(const Canvas& other);
    void write_pixel(int x, int y, const Colour& pixel);
    const Colour& pixel_at(int x, int y) const;
    string to_ppm() const;
    void write_ppm(const char *path) const;
    static int clamp_rgb(double x);

private:
    int width;
    int height;
    Colour** pixels;
};


#endif
