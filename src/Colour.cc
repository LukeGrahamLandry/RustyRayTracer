#include "Colour.h"

Colour::Colour(float red, float green, float blue) {
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
    return Colour(red + other.red, blue + other.blue, green + other.green);
}

Colour Colour::subtract(const Colour &other) const {
    return Colour(red - other.red, blue - other.blue, green - other.green);
}

Colour Colour::scale(float s) const {
    return Colour(red * s, blue * s, green * s);
}

Colour Colour::multiply(const Colour &other) const {
    return Colour(red * other.red, blue * other.blue, green * other.green);
}
