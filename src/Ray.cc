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

}

Intersections::Intersections(initializer_list<Intersection> group) {
    for (Intersection i : group){
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

int Intersections::count() {
    return (int) intersections.size();
}

Intersection Intersections::get(int index) {
    return *intersections[index];
}

Intersection Intersections::hit() {
    for (int i=0;i<intersections.size();i++){
        if (intersections[i]->t >= 0){
            return *intersections[i];
        }
    }

#ifdef DEBUG_CHECKS
    error() << "One does not simply hit the unhittable." << endl;
#endif
    return Intersection();
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

Intersection::Intersection(double tIn, Sphere &objectIn) {
    t = tIn;
    object = &objectIn;
}

Intersection::Intersection() {
    t = 0;
    object = nullptr;
    hasPreComputed = false;
}

bool Intersection::equals(const Intersection& other) const {
    return almostEqual(t, other.t) && (object == other.object || object->equals(*other.object));
}

Intersection& Intersection::prepare_computations(const Ray &ray) {
    hasPreComputed = true;
    point = ray.position(t);
    eyev = ray.direction.negate();
    normalv = object->normal_at(point);
    inside = normalv.dot(eyev) < 0;
    if (inside) normalv = normalv.negate();
    over_point = point.add(normalv.scale(EPSILON()));
    return *this;
}

Intersection::Intersection(const Intersection &other) {
    t = other.t;
    object = other.object;
    hasPreComputed = other.hasPreComputed;
    eyev = other.eyev;
    normalv = other.normalv;
    inside = other.inside;
}
