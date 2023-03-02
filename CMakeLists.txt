cmake_minimum_required(VERSION 3.24)
project(raytracer)

set(CMAKE_CXX_STANDARD 14)

include_directories(src)

find_package(SDL2 REQUIRED COMPONENTS SDL2)

add_executable(raytracer
        src/Tuple.cc src/Tuple.h src/Colour.h src/Colour.cc src/Canvas.h src/Canvas.cc src/main.cc src/common.h src/Matrix.h src/Matrix.cc src/Ray.h src/Ray.cc src/demo.cc src/shapes/Sphere.h src/shapes/Sphere.cc src/Material.h src/Material.cc src/World.h src/World.cc src/Camera.cc src/Camera.h src/shapes/Shape.cc src/shapes/Shape.h src/shapes/Plane.h src/shapes/Plane.cc)

add_executable(raytracer_tests
        src/Tuple.cc src/Tuple.h src/Colour.h src/Colour.cc src/Canvas.h src/Canvas.cc src/common.h src/Matrix.h src/Matrix.cc src/tests.cc src/Ray.h src/Ray.cc src/demo.cc src/shapes/Sphere.h src/shapes/Sphere.cc src/Material.h src/Material.cc src/World.h src/World.cc src/Camera.cc src/Camera.h src/shapes/Shape.cc src/shapes/Shape.h src/shapes/Plane.h src/shapes/Plane.cc)

target_link_libraries(raytracer PRIVATE SDL2::SDL2)
target_link_libraries(raytracer_tests PRIVATE SDL2::SDL2)
