#ifndef RAYTRACER_TUPLE_H
#define RAYTRACER_TUPLE_H

#include "common.h"

class Tuple {
public:
    Tuple();
    Tuple(const Tuple& other);

    float values[4];

    Tuple(float x, float y, float z, float w);
    bool isPoint() const;
    bool isVector() const;
    Tuple add(const Tuple& other) const;
    Tuple subtract(const Tuple &other) const;
    Tuple negate() const;
    Tuple scale(float s) const;
    Tuple divide(float s) const;
    float magnitude() const;
    Tuple normalize() const;
    float dot(const Tuple &other) const;
    Tuple cross(const Tuple &other) const;

    bool equals(const Tuple &other) const {
        for (int i=0;i<4;i++){
            if (!almostEqual(get(i), other.get(i))) return false;
        }
        return true;
    }

    inline void set(int i, float value){
        values[i] = value;
    }
    inline float get(int i) const {
        return values[i];
    }
    inline float x() const {
        return values[0];
    }
    inline float y() const {
        return values[1];
    }
    inline float z() const {
        return values[2];
    }
    inline float w() const {
        return values[3];
    }

    void print() const;
};

class Point : public Tuple {
public:
    Point(float x, float y, float z): Tuple(x, y, z, 1){

    }
};

class Vector : public Tuple {
public:
    Vector(float x, float y, float z): Tuple(x, y, z, 0){

    }
};

#endif
