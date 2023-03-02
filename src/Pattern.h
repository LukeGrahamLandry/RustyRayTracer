#ifndef RAYTRACER_PATTERN_H
#define RAYTRACER_PATTERN_H

#include "Colour.h"
#include "Matrix.h"

class Shape;

class Pattern {
public:
    MemoMatrix transform;
    Pattern() {
        transform = Transformation::identity();
    }

    virtual ~Pattern() = default;
    virtual Colour local_pattern_at(const Tuple& pattern_point) const = 0;
    virtual Colour pattern_at(Shape* object, const Tuple& world_point) const;
    virtual Pattern *copy() const = 0;

    virtual Pattern* copy_to(Pattern* p) const{
        p->transform = transform;
        return p;
    }

    Colour object_pattern_at(const Tuple& object_point) const {
        return local_pattern_at(transform.inverse().multiply(object_point));
    }
};


class SolidPattern : public Pattern {
public:
    Colour color;
    SolidPattern(const Colour& c): color{c} {}

    Colour pattern_at(Shape* object, const Tuple& world_point) const override {
        return color;
    }

    Colour local_pattern_at(const Tuple& pattern_point) const override {
        return color;
    }

    Pattern* copy() const override {
        return copy_to(new SolidPattern(color));
    }
};

class DoublePattern : public Pattern {
public:
    Pattern* a;
    Pattern* b;
    DoublePattern(const Pattern& a, const Pattern& b) {
        this->a = a.copy();
        this->b = b.copy();
    }

    DoublePattern(const Colour& a, const Colour& b) {
        this->a = new SolidPattern(a);
        this->b = new SolidPattern(b);
    }

    ~DoublePattern() override {
        delete a;
        delete b;
    }
};

class StripePattern : public DoublePattern {
public:
    StripePattern(const Pattern& a, const Pattern& b) : DoublePattern(a, b){}
    StripePattern(const Colour& a, const Colour& b) : DoublePattern(a, b){}

    Colour local_pattern_at(const Tuple& pattern_point) const override {
        return (int) floor(pattern_point.x()) % 2 == 0 ? a->object_pattern_at(pattern_point) : b->object_pattern_at(pattern_point);
    }

    Pattern* copy() const override {
        return copy_to(new StripePattern(*a, *b));
    }
};

class GradientPattern : public DoublePattern {
public:
    GradientPattern(const Pattern& a, const Pattern& b) : DoublePattern(a, b){}
    GradientPattern(const Colour& a, const Colour& b) : DoublePattern(a, b){}

    Colour local_pattern_at(const Tuple& pattern_point) const override {
        Colour c_a = a->object_pattern_at(pattern_point);
        Colour c_b = b->object_pattern_at(pattern_point);
        Colour distance = c_b.subtract(c_a);
        double t = pattern_point.x(); // (pattern_point.x() + 1) * 0.5;  // pattern_point.x() - floor(pattern_point.x());
        return c_a.add(distance.scale(t));
    }

    Pattern* copy() const override {
        return copy_to(new GradientPattern(*a, *b));
    }
};

class RepeatingPattern : public Pattern {
public:
    Pattern* a;
    RepeatingPattern(const Pattern& a) {
        this->a = a.copy();
    }

    RepeatingPattern(const Colour& a) {
        this->a = new SolidPattern(a);
    }

    ~RepeatingPattern() override {
        delete a;
    }

    Colour local_pattern_at(const Tuple& pattern_point) const override {
        return a->object_pattern_at(Point(pattern_point.x() - floor(pattern_point.x()), pattern_point.y() - floor(pattern_point.y()), pattern_point.z() - floor(pattern_point.z())));
    }

    Pattern* copy() const override {
        return copy_to(new RepeatingPattern(*a));
    }
};

#include "shapes/Shape.h"

#endif
