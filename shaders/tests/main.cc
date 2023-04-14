#include "tests_common.h"

extern "C" {
// This gets called from rust.
int run_tests(){
    cout << "Running tests..." << endl;
    Camera c;
    rs::resize((rs::Camera *)(&c), 10, 10);
    cout << c.hsize << endl;
    return 0;
}
}
