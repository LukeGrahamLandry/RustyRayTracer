#include "Pattern.h"

 Colour Pattern::pattern_at(Shape* object, const Tuple& world_point) const {
    Tuple object_point = object->transform.inverse().multiply(world_point);
    return object_pattern_at(object_point);
}