############################################################################
# This isn't actually used for anything... but mother cares not for Xcode. #
# I just want CLion to index stuff so I can click around while debugging.  #
# See ../build.rs for the actual build script.                             #
############################################################################

add_compile_definitions(NOT_BUILDING_AS_MSL)

cmake_minimum_required(VERSION 3.24)
project(shaders)

set(CMAKE_CXX_STANDARD 14)

include_directories(src)

add_executable(unused_cpu_shaders
        src/common.h
        src/lib.cc
        src/material.cc
        src/material.h
        src/ray.cc
        src/ray.h
        src/shapes.cc
        src/shapes.h
        src/world.cc
        src/world.h)
