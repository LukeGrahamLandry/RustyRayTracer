#ifndef RAYTRACER_MATERIAL_H
#define RAYTRACER_MATERIAL_H

#include "Colour.h"
#include "Tuple.h"

class PointLight {
public:
    Tuple position;
    Colour intensity;
    PointLight(const Tuple &positionIn, const Colour &intensityIn);
};

class Shape;
class Pattern;

class Material {
public:
    Colour color;

    double ambient;
    double diffuse;
    double specular;

    double shininess;

    Pattern* pattern;

    Material();
    Colour lighting(const PointLight& light, Shape* object, const Tuple& position, const Tuple& eye_vector, const Tuple& normal_vector) const;
    Colour lighting(const PointLight& light, Shape* object, const Tuple& position, const Tuple& eye_vector, const Tuple& normal_vector, bool in_shadow) const;

    void setPattern(const Pattern& p);
    bool equals(const Material& material) const;
};

#include "shapes/Shape.h"
#include "Pattern.h"

#endif
