#include "Colour.h"

Colour::Colour(double red, double green, double blue) {
    this->red = red;
    this->green = green;
    this->blue = blue;
}

Colour::Colour() {
    red = 0;
    green = 0;
    blue = 0;
}

Colour Colour::add(const Colour &other) const {
    return Colour(red + other.red, green + other.green, blue + other.blue);
}

Colour Colour::subtract(const Colour &other) const {
    return Colour(red - other.red, green - other.green, blue - other.blue);
}

Colour Colour::scale(double s) const {
    return Colour(red * s, green * s, blue * s);
}

Colour Colour::multiply(const Colour &other) const {
    return Colour(red * other.red, green * other.green, blue * other.blue);
}
