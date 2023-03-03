#include "Material.h"

PointLight::PointLight(const Tuple& positionIn, const Colour& intensityIn) {
    position = positionIn;
    intensity = intensityIn;
}

Material::Material() {
    color = Colour(1, 1, 1);
    ambient = 0.1;
    diffuse = 0.9;
    specular = 0.9;
    shininess = 200.0;
    pattern = nullptr;
}

Colour Material::lighting(const PointLight& light, Shape* object, const Tuple& position, const Tuple& eye_vector, const Tuple& normal_vector, bool in_shadow) const {
    if (pattern == nullptr) return lighting(light, position, eye_vector, normal_vector, in_shadow, color);
    else return lighting(light, position, eye_vector, normal_vector, in_shadow, pattern->pattern_at(object, position));
}

Colour Material::lighting(const PointLight& light, const Tuple& position, const Tuple& eye_vector, const Tuple& normal_vector, bool in_shadow, const Colour& object_color) const {
    Colour base_colour = object_color.multiply(light.intensity);
    Colour ambient_colour = object_color.scale(ambient);

    if (in_shadow) return ambient_colour;

    Tuple light_direction = light.position.subtract(position).normalize();
    double cos_light_to_normal = light_direction.dot(normal_vector);  // Since both are normalized

    Colour diffuse_colour;
    Colour specular_colour;
    if (cos_light_to_normal >= 0){
        diffuse_colour = base_colour.scale(diffuse * cos_light_to_normal);

        Tuple reflection_direction = light_direction.negate().reflect(normal_vector);
        double cos_reflect_to_eye = reflection_direction.dot(eye_vector);  // Since both are normalized

        if (cos_reflect_to_eye >= 0){
            double factor = pow(cos_reflect_to_eye, shininess);
            specular_colour = light.intensity.scale(specular * factor);
        }
    }

    return ambient_colour.add(diffuse_colour).add(specular_colour);
}

bool Material::equals(const Material& other) const {
    // TODO: include pattern
    return color.equals(other.color) && almostEqual(ambient, other.ambient) && almostEqual(diffuse, other.diffuse) && almostEqual(specular, other.specular) && almostEqual(shininess, other.shininess);
}

void Material::setPattern(const Pattern& p) {
    pattern = p.copy();
}
