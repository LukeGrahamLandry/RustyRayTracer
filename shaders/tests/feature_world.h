#include "tests_common.h"

TEST(intersect_a_world_with_a_ray)
    World world;
    default_world(&world);
    Ray ray = Ray(Point(0, 0, -5), Vector(0, 0, 1));
    Intersections xs;
    world.intersect(ray, xs);
    assert_eq(xs.count, 4);
    assert_eq(xs.hits[0].t, 4.0f);
    assert_eq(xs.hits[1].t, 4.5f);
    assert_eq(xs.hits[2].t, 5.5f);
    assert_eq(xs.hits[3].t, 6.0f);
}

void feature_world(){
    intersect_a_world_with_a_ray();
}