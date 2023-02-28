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

class Material {
public:
    Colour color;

    double ambient;
    double diffuse;
    double specular;

    double shininess;

    Material();
    Colour lighting(const PointLight& light, const Tuple& position, const Tuple& eye_vector, const Tuple& normal_vector) const;
    Colour lighting(const PointLight& light, const Tuple& position, const Tuple& eye_vector, const Tuple& normal_vector, bool in_shadow) const;

    bool equals(const Material& material) const;
};


#endif
