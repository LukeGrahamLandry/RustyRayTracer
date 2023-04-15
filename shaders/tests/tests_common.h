#ifndef TESTS_COMMON_H
#define TESTS_COMMON_H

#include "../src/common.h"
#include "../src/material.h"
#include "../src/world.h"
#include "../src/ray.h"
#include "../src/shapes.h"

// Undo my clever macros because these words are used in the standard library.
// Since the tests only run on the CPU, they don't need to be valid MSL.
#undef device
#undef constant
#undef thread

#include <iostream>
#include <string>
using namespace std;

#define usize uintptr_t
using BackingWorldPtr = void*;

// Rust functions to set up the context some tests need.
extern "C" {
void resize(Camera* camera, usize hsize, usize vsize);
BackingWorldPtr create_default_world(World* out);
void drop_world(BackingWorldPtr ptr);
void get_structs_repr(usize count, usize* sizes_out, usize* alignments_out);
}

// Whenever possible, create an unnecessary untyped DSL with the preprocessor!
// This just gives me a nice little reflection where the function can access its name as a string without typing it twice.
#define TEST(name) void name() { start_test(#name);

static int tests_passed = 0;
static int tests_failed = 0;

static int current_subtest_index = 0;
static string current_test;
void start_test(const string& name){
    current_test = name;
    current_subtest_index = 0;
}

// A bunch of annoying equality boilerplate

template<class T>
void assert_eq(T a, T b){
    if (a != b){
        cout << "Failed: " << current_test << " (" << current_subtest_index << ").\n        " << a << " != " << b << "." << endl;
        tests_failed++;
    } else {
        tests_passed++;
    }
    current_subtest_index++;
}

bool almost_equal(float a, float b){
    return abs(a - b) < EPSILON;
}

ostream& operator<<(ostream& s, const float3& v) {
    s << "(" << v.x << ", " << v.y << ", " << v.z << ")";
    return s;
}

ostream& operator<<(ostream& s, const float4& v) {
    s << "(" << v.x << ", " << v.y << ", " << v.z << ", " << v.w << ")";
    return s;
}

void assert_eq(float3 a, float3 b){
    bool match = almost_equal(a.x, b.x) && almost_equal(a.y, b.y) && almost_equal(a.z, b.z);
    if (!match){
        cout << "Failed: " << current_test << ".\n        " << a << " != " << b << "." << endl;
        tests_failed++;
    } else {
        tests_passed++;
    }
}

void assert_eq(float4 a, float4 b){
    bool match = almost_equal(a.x, b.x) && almost_equal(a.y, b.y) && almost_equal(a.z, b.z) && almost_equal(a.w, b.w);
    if (!match){
        cout << "Failed: " << current_test << ".\n        " << a << " != " << b << "." << endl;
        tests_failed++;
    } else {
        tests_passed++;
    }
}

#endif
