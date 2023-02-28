#define DEBUG_CHECKS

#include "cmath"
#include <iostream>

#ifndef COMMON_H
#define COMMON_H

using namespace std;

inline double EPSILON(){
    return 0.000001;
}

inline bool almostEqual(double a, double b) {
    return abs(a - b) < EPSILON();
}

inline ostream& error() {
    return cerr;
}


#endif