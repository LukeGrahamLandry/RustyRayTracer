#ifndef RAYTRACER_TUPLE_H
#define RAYTRACER_TUPLE_H

#include "common.h"

class Tuple {
public:
    Tuple();
    Tuple(const Tuple& other);

    double values[4];

    Tuple(double x, double y, double z, double w);
    bool isPoint() const;
    bool isVector() const;
    Tuple add(const Tuple& other) const;
    Tuple subtract(const Tuple &other) const;
    Tuple negate() const;
    Tuple scale(double s) const;
    Tuple divide(double s) const;
    double magnitude() const;
    Tuple normalize() const;
    double dot(const Tuple &other) const;
    Tuple cross(const Tuple &other) const;
    Tuple reflect(const Tuple& normal) const;

    bool equals(const Tuple &other) const {
        for (int i=0;i<4;i++){
            if (!almostEqual(get(i), other.get(i))) return false;
        }
        return true;
    }

    inline void set(int i, double value){
        values[i] = value;
    }
    inline double get(int i) const {
        return values[i];
    }
    inline double x() const {
        return values[0];
    }
    inline double y() const {
        return values[1];
    }
    inline double z() const {
        return values[2];
    }
    inline double w() const {
        return values[3];
    }

    void print() const;
};

class Point : public Tuple {
public:
    Point(double x, double y, double z): Tuple(x, y, z, 1){

    }
};

class Vector : public Tuple {
public:
    Vector(double x, double y, double z): Tuple(x, y, z, 0){

    }
};

#endif
