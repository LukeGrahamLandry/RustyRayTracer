#include "material.h"


// https://en.wikipedia.org/wiki/Phong_reflection_model
float3 Material::lighting(PointLight light, float4 position, float4 eye_vector, float4 normal_vector, bool in_shadow) const {
    float3 base_colour = colour * light.intensity;
    float3 ambient_colour = colour * ambient;

    if (in_shadow) return ambient_colour;

    float4 light_direction = normalize(light.position - position);
    float cos_light_to_normal = dot(light_direction, normal_vector);  // Since both are normalized

    float3 diffuse_colour = float3(0);
    float3 specular_colour = float3(0);
    if (cos_light_to_normal >= 0){
        diffuse_colour = base_colour * diffuse * cos_light_to_normal;

        float4 reflection_direction = reflect(-light_direction, normal_vector);
        float cos_reflect_to_eye = dot(reflection_direction, eye_vector);  // Since both are normalized

        if (cos_reflect_to_eye >= 0){
            float factor = pow(cos_reflect_to_eye, shininess);
            specular_colour = light.intensity * specular * factor;
        }
    }

    return ambient_colour + diffuse_colour + specular_colour;
}
