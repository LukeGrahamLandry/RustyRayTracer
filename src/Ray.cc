#include "Ray.h"

Ray::Ray(): Ray(Point(0, 0, 0), Vector(1, 0, 0)) {

}

Ray::Ray(const Tuple& originIn, const Tuple& directionIn) {
    origin = originIn;
    direction = directionIn;
}

Intersections::~Intersections() {
    for (Intersection* i : intersections){
        delete i;
    }
}

Tuple Ray::position(double t) const {
    return origin.add(direction.scale(t));
}

Ray Ray::transform(const Matrix& transformation) const {
    return Ray(transformation.multiply(origin), transformation.multiply(direction));
}

Intersections::Intersections() {
    intersections.reserve(5);
}

Intersections::Intersections(initializer_list<Intersection> group) {
    for (const Intersection& i : group){
        add(i);
    }
}

void Intersections::add(const Intersection& intersection) {
    Intersection* toAdd = new Intersection(intersection);
    for (int i=0;i<intersections.size();i++){
        Intersection* current = intersections[i];
        if (current->t >= toAdd->t){
            intersections[i] = toAdd;
            toAdd = current;
        }
    }
    intersections.push_back(toAdd);
}

int Intersections::count() const {
    return (int) intersections.size();
}

const Intersection& Intersections::get(int index) {
    return *intersections[index];
}

const Intersection& Intersections::hit() {
    for (int i=0;i<intersections.size();i++){
        if (intersections[i]->t >= 0){
            return *intersections[i];
        }
    }

    throw runtime_error("You must check hasHit before calling Intersections::hit");
}

bool Intersections::hasHit() {  // Inefficient that <hit> has to do the loop again
    for (int i=0;i<intersections.size();i++){
        if (intersections[i]->t >= 0){
            return true;
        }
    }
    return false;
}

void Intersections::addAll(const Intersections& hits) {
    for (Intersection* hit : hits.intersections){
        add(*hit);
    }
}

bool Intersection::equals(const Intersection& other) const {
    return almostEqual(t, other.t) && (object == other.object || object->equals(*other.object));
}

IntersectionComps Intersection::prepare_computations(const Ray &ray) const {
    IntersectionComps comps;
    comps.t = t;
    comps.object = object;
    comps.point = ray.position(t);
    comps.eyev = ray.direction.negate();
    comps.normalv = object->normal_at(comps.point);
    comps.inside = comps.normalv.dot(comps.eyev) < 0;
    if (comps.inside) comps.normalv = comps.normalv.negate();

    // Used for is_shadowed checks to prevent shadow acne
    comps.over_point = comps.point.add(comps.normalv.scale(EPSILON()));
    return comps;
}

Intersection::Intersection(const Intersection &other) {
    t = other.t;
    object = other.object;
}

Intersection::Intersection(double tIn, Shape& objectIn) {
    t = tIn;
    object = &objectIn;
}

Intersection::Intersection() {
    t = 0;
    object = nullptr;
}