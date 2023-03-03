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
    double ambient;   // 0 to 1
    double diffuse;   // 0 to 1
    double specular;  // 0 to 1
    double shininess;
    Pattern* pattern;

    Material();
    void setPattern(const Pattern& p);
    bool equals(const Material& material) const;

    // Does the actual work.
    Colour lighting(const PointLight& light, const Tuple& position, const Tuple& eye_vector, const Tuple& normal_vector, bool in_shadow, const Colour& object_color) const;

    // For normal usage
    Colour lighting(const PointLight& light, Shape* object, const Tuple& position, const Tuple& eye_vector, const Tuple& normal_vector, bool in_shadow=false) const;

    // For tests before introducing shadows or patterns.
    Colour lighting(const PointLight& light, const Tuple& position, const Tuple& eye_vector, const Tuple& normal_vector, bool in_shadow=false) const {
        return lighting(light, position, eye_vector, normal_vector, in_shadow, color);
    }

};

#include "shapes/Shape.h"
#include "Pattern.h"

#endif
