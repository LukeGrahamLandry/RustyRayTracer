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

static int tests_passed = 0;
static int tests_failed = 0;

static string current_test;
void start_test(const string& name){
    current_test = name;
}

template<class T>
void assert_eq(T a, T b){
    if (a != b){
        cout << "Failed: " << current_test << ".\n        " << a << " != " << b << "." << endl;
        tests_failed++;
    } else {
        tests_passed++;
    }
}

// Whenever possible, create an unnecessary untyped DSL with the preprocessor!
#define TEST(name) void name() { start_test(#name);

// Rust functions to set up the context some tests need.
extern "C" {
void resize(Camera* camera, usize hsize, usize vsize);
void default_world(World* out);
void get_structs_repr(usize count, usize* sizes_out, usize* alignments_out);
}

#endif
