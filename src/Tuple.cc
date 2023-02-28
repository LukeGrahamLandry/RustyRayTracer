#include "Tuple.h"

Tuple::Tuple(double x, double y, double z, double w) {
    set(0, x);
    set(1, y);
    set(2, z);
    set(3, w);
    
#ifdef DEBUG_CHECKS
    if (w != 0 && w != 1) error() << "One does not simply create a tuple that is not a vector or a point." << endl;
#endif
}

bool Tuple::isPoint() const {
    return w() == 1;
}

bool Tuple::isVector() const {
    return w() == 0;
}

Tuple Tuple::add(const Tuple& other) const {
#ifdef DEBUG_CHECKS
    if (other.isPoint()) error() << "One does not simply add point to something." << endl;
#endif

    return Tuple(x() + other.x(), y() + other.y(), z() + other.z(), w() + other.w());
}

Tuple Tuple::subtract(const Tuple& other) const {
#ifdef DEBUG_CHECKS
    if (isVector() && other.isPoint()) error() << "One does not simply() subtract a vector from a point." << endl;
#endif

    return Tuple(x() - other.x(), y() - other.y(), z() - other.z(), w() - other.w());
}

Tuple Tuple::negate() const {
#ifdef DEBUG_CHECKS
    if (isPoint()) error() << "One does not simply negate a point." << endl;
#endif

    return Tuple(-x(), -y(), -z(), -w());
}

Tuple Tuple::scale(double s) const {
#ifdef DEBUG_CHECKS
    if (isPoint()) error() << "One does not simply scale a point." << endl;
#endif

    return Tuple(x() * s, y() * s, z() * s, w() * s);
}

Tuple Tuple::divide(double s) const {
#ifdef DEBUG_CHECKS
    if (isPoint()) error() << "One does not simply scale a point." << endl;
#endif

    return Tuple(x() / s, y() / s, z() / s, w() / s);
}

double Tuple::magnitude() const {
#ifdef DEBUG_CHECKS
    if (isPoint()) error() << "One does not simply ask the length of a point." << endl;
#endif

    double lengthSq = 0;
    for (int i=0;i<4;i++){
        lengthSq += get(i) * get(i);
    }

    return sqrt(lengthSq);
}

Tuple Tuple::normalize() const {
    double length = magnitude();

#ifdef DEBUG_CHECKS
    if (isPoint()) error() << "One does not simply normalize a point." << endl;
    else if (length == 0) error() << "One does not simply normalize the zero vector." << endl;
#endif

    return Tuple(x() / length, y() / length, z() / length, w() / length);
}

double Tuple::dot(const Tuple& other) const {
#ifdef DEBUG_CHECKS
    if (isPoint() || other.isPoint()) error() << "One does not simply take the dot product with a point." << endl;
#endif

    double v = 0;
    for (int i=0;i<4;i++){
        v += get(i) * other.get(i);
    }

    return v;
}

Tuple Tuple::cross(const Tuple& other) const {
#ifdef DEBUG_CHECKS
    if (isPoint() || other.isPoint()) error() << "One does not simply take the cross product with a point." << endl;
#endif

    return Tuple(y() * other.z() - z() * other.y(),
                 z() * other.x() - x() * other.z(),
                 x() * other.y() - y() * other.x(),
                 w());
}

Tuple::Tuple() : Tuple(0, 0, 0, 0){

}

void Tuple::print() const {
    cout << "(" << x() << ", " << y() << ", " << z() << ", " << w() << ")" << endl;
}

Tuple::Tuple(const Tuple &other) {
    for (int i=0;i<4;i++){
        set(i, other.get(i));
    }
}

Tuple Tuple::reflect(const Tuple& normal) const {
    return subtract(normal.scale(2 * dot(normal)));
}
