#include "Ray.h"

Ray::Ray(): Ray(Point(0, 0, 0), Vector(1, 0, 0)) {

}

Ray::Ray(const Tuple& originIn, const Tuple& directionIn) {
    origin = originIn;
    direction = directionIn;
}

Tuple Ray::position(float t) const {
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

void Intersections::add(Intersection intersection) {
    for (int i=0;i<intersections.size();i++){
        Intersection current = intersections[i];
        if (current.t >= intersection.t){
            intersections[i] = intersection;
            intersection = current;
        }
    }

    intersections.push_back(intersection);
}

int Intersections::count() {
    return (int) intersections.size();
}

Intersection Intersections::get(int index) {
    return intersections[index];
}

Intersection Intersections::hit() {
    for (int i=0;i<intersections.size();i++){
        if (intersections[i].t >= 0){
            return intersections[i];
        }
    }

#ifdef DEBUG_CHECKS
    error() << "One does not simply hit the unhittable." << endl;
#endif
    return Intersection();
}

bool Intersections::hasHit() {  // Inefficient that <hit> has to do the loop again
    for (int i=0;i<intersections.size();i++){
        if (intersections[i].t >= 0){
            return true;
        }
    }
    return false;
}

Intersection::Intersection(float tIn, Sphere &objectIn) {
    t = tIn;
    object = &objectIn;
}

Intersection::Intersection() {
    t = 0;
    object = nullptr;
}

bool Intersection::equals(const Intersection& other) const {
    return almostEqual(t, other.t) && (object == other.object || object->equals(*other.object));
}
