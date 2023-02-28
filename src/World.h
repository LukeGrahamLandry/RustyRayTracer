#ifndef RAYTRACER_WORLD_H
#define RAYTRACER_WORLD_H

#include "Material.h"
#include "shapes/Sphere.h"

class World {
public:
    vector<PointLight*> lights;
    vector<Sphere*> objects;
    World();
    ~World();

    Intersections intersect(const Ray& ray) const;
    Colour shade_hit(const Intersection& hit) const;
    Sphere getShape(int index);
    Colour color_at(const Ray& ray) const;
    void addLight(const PointLight& light);

    static World default_world();

    void addShape(const Sphere& sphere);

    bool is_shadowed(const Tuple& point, const PointLight &light) const;

    PointLight getLight(int index);
};


#endif
