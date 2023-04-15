#include "tests_common.h"

static World world;

TEST(intersect_a_world_with_a_ray)
    Ray ray = Ray(Point(0, 0, -5), Vector(0, 0, 1));
    Intersections xs;
    world.intersect(ray, xs);
    assert_eq(xs.count, 4);
    assert_eq(xs.hits[0].t, 4.0f);
    assert_eq(xs.hits[1].t, 4.5f);
    assert_eq(xs.hits[2].t, 5.5f);
    assert_eq(xs.hits[3].t, 6.0f);
}

TEST(shading_an_intersection)
    Ray ray = Ray(Point(0, 0, -5), Vector(0, 0, 1));
    Intersections xs;
    Intersection i = {4, 0};
    Comps comps = world.prepare_comps(i, ray, xs);
    float3 colour = world.shade_hit(comps);
    assert_eq(colour, make_float3(0.38066, 0.47583, 0.2855));
}

// TODO: shading_an_intersection_from_the_inside

TEST(the_color_when_a_ray_misses)
    Ray ray = Ray(Point(0, 0, -5), Vector(0, 1, 0));
    float3 colour = world.colour_at(ray);
    assert_eq(colour, make_float3(0, 0, 0));
}

TEST(the_color_when_a_ray_hits)
    Ray ray = Ray(Point(0, 0, -5), Vector(0, 0, 1));
    float3 colour = world.colour_at(ray);
    assert_eq(colour, make_float3(0.38066, 0.47583, 0.2855));
}

// TODO: The color with an intersection behind the ray

void is_shadowed(){
#define SHADOW(p, expected, name) \
    start_test(#name); \
    assert_eq(expected, world.is_shadowed(world.lights[0].position, p))

    SHADOW(Point(0, 10, 0), false, there_is_no_shadow_when_nothing_is_collinear_with_the_point);
    SHADOW(Point(10, -10, 10), true, the_shadow_when_an_object_is_between_the_point_and_the_light);
    SHADOW(Point(-20, 20, 20), false, there_is_no_shadow_when_an_object_is_behind_the_light);
    SHADOW(Point(-2, 2, 2), false, there_is_no_shadow_when_an_object_is_behind_the_point);

#undef SHADOW
}

void feature_world(){
    BackingWorldPtr world_box = create_default_world(&world);

    intersect_a_world_with_a_ray();
    shading_an_intersection();
    the_color_when_a_ray_misses();
    the_color_when_a_ray_hits();
    is_shadowed();

    drop_world(world_box);
}