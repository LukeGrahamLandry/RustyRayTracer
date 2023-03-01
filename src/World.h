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

    Intersections intersect(const Ray& ray) const;
    Colour shade_hit(const IntersectionComps& hit) const;
    Shape* getShape(int index);
    Colour color_at(const Ray& ray) const;
    void addLight(PointLight& light);

    static World default_world();

    void addShape(Shape& shape);

    bool is_shadowed(const Tuple& point, PointLight* light) const;

    PointLight* getLight(int index);
};


#endif
