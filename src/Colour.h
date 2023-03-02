#ifndef RAYTRACER_COLOUR_H
#define RAYTRACER_COLOUR_H

#include "common.h"

class Colour {
public:
    double red;
    double green;
    double blue;

    Colour(double red, double green, double blue);
    Colour();
    Colour add(const Colour& other) const;
    Colour subtract(const Colour& other) const;
    Colour scale(double s) const;
    Colour multiply(const Colour& other) const;
    bool equals(const Colour& other) const {
        return almostEqual(red, other.red) && almostEqual(green, other.green) && almostEqual(blue, other.blue);
    }
};


#endif
