#include "World.h"

World::World() {

}

World::~World() {
    for (Shape* s : objects){
        delete s;
    }
    for (PointLight* l : lights){
        delete l;
    }
}

World World::default_world() {
    World world;

    world.addLight(PointLight(Point(-10, 10, -10), Colour(1, 1, 1)));
    Sphere sphere1;
    sphere1.material.color = Colour(0.8, 1.0, 0.6);
    sphere1.material.diffuse = 0.7;
    sphere1.material.specular = 0.2;
    world.addShape(Sphere(sphere1));
    Sphere sphere2;
    sphere2.set_transform(Transformation::scaling(0.5, 0.5, 0.5));
    world.addShape(Sphere(sphere2));

    return world;
}

void World::intersect(const Ray &ray, Intersections& result) const {
    for (Shape* obj : objects){
        obj->intersect(ray, result);
    }
}

bool World::is_shadowed(const Tuple& point, PointLight* light) const {
    Tuple light_direction = light->position.subtract(point);
    Ray ray = Ray(point, light_direction.normalize());
    Intersections hits;
    intersect(ray, hits);
    // Make sure the hit is not behind the light.
    return hits.hasHit() && hits.hit().t < light_direction.magnitude();
}

Colour World::shade_hit(const IntersectionComps& hit) const {
    Colour total;
    for (PointLight* light : lights){
        Colour current = hit.object->material.lighting(*light, hit.object, hit.point, hit.eyev, hit.normalv, is_shadowed(hit.over_point, light));
        total = total.add(current);
    }
    return total;
}

Shape* World::getShape(int index) {
    return objects[index];
}

PointLight* World::getLight(int index) {
    return lights[index];
}

Colour World::color_at(const Ray &ray) const {
    Intersections hits;
    intersect(ray, hits);
    if (!hits.hasHit()) return Colour();
    IntersectionComps hit = hits.hit().prepare_computations(ray);
    return shade_hit(hit);
}

void World::addLight(const PointLight& light) {
    lights.push_back(new PointLight(light));
}

void World::addShape(const Shape& sphere) {
    objects.push_back(sphere.copy());
}

