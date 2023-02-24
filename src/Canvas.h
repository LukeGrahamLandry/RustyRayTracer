#ifndef RAYTRACER_CANVAS_H
#define RAYTRACER_CANVAS_H

#include "Colour.h"

class Canvas {
public:
    Canvas(int width, int height);
    ~Canvas();
    void write_pixel(int x, int y, const Colour& pixel);
    const Colour & pixel_at(int x, int y) const;
    string to_ppm() const;
    void write_ppm(const char *path) const;
private:
    int width;
    int height;
    Colour** pixels;
};


#endif
