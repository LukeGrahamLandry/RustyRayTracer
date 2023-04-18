#ifndef world_h
#define world_h

#include "common.h"
#include "shapes.h"
#include "ray.h"

typedef struct PointLight {
    float4 position;
    float3 intensity;
} PointLight;

typedef struct {
    Camera camera;
    uint32_t shape_count;
    uint32_t light_count;
} ShaderInputs;

typedef struct Comps {
    float t;
    float n1;
    float n2;
    bool inside;
    Material material;
    float4 point;
    float4 eyev;
    float4 normalv;
    float4 over_point;
    float4 reflectv;
    float4 under_point;
} Comps;

typedef struct WorldView {
    const device Shape* shapes;
    const device PointLight* lights;
    WorldView(const device Shape* s, const device PointLight* l, const constant ShaderInputs& i)
            : shapes(s), lights(l), inputs(i) {};

    ShaderInputs inputs;

    float3 colour_at(const thread Ray& ray) const;
    void intersect(const thread Ray& ray, thread Intersections& hits) const;
    float3 shade_hit(const thread Comps& comps) const;
    bool is_shadowed(const thread float4& light_pos, const thread float4& hit_pos) const;
    Comps prepare_comps(const thread Intersection& hit, const thread Ray& ray, const thread Intersections& xs) const;
    void refraction_path(thread Comps&, const thread Intersection&, const thread Intersections&) const;
} WorldView;

#endif 
