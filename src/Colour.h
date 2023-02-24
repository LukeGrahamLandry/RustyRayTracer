#ifndef RAYTRACER_COLOUR_H
#define RAYTRACER_COLOUR_H

#include "common.h"

class Colour {
public:
    float red;
    float green;
    float blue;

    Colour(float red, float green, float blue);
    Colour();
    Colour add(const Colour& other) const;
    Colour subtract(const Colour& other) const;
    Colour scale(float s) const;
    Colour multiply(const Colour& other) const;
};


#endif //RAYTRACER_COLOUR_H
