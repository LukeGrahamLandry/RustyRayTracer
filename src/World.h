#ifndef RAYTRACER_WORLD_H
#define RAYTRACER_WORLD_H

#include "Material.h"
#include "shapes/Shape.h"
#include "shapes/Sphere.h"

class World {
public:
    vector<PointLight*> lights;
    vector<Shape*> objects;
    World();
    ~World();

    Intersections intersect(const Ray& ray) const {
        Intersections hits;
        intersect(ray, hits);
        return hits;
    }
    void intersect(const Ray& ray, Intersections& intersections) const;
    Colour shade_hit(const IntersectionComps& hit) const;
    Shape* getShape(int index);
    Colour color_at(const Ray& ray) const;
    void addLight(const PointLight& light);

    static World default_world();

    void addShape(const Shape& shape);

    bool is_shadowed(const Tuple& point, PointLight* light) const;

    PointLight* getLight(int index);
};


#endif
