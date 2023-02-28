#define DEBUG_CHECKS

#include "cmath"
#include <iostream>

#ifndef COMMON_H
#define COMMON_H

using namespace std;

inline float EPSILON(){
    return 0.0000001;
}

inline bool almostEqual(float a, float b) {
    return abs(a - b) < EPSILON();
}

inline ostream& error() {
    return cerr;
}


#endif