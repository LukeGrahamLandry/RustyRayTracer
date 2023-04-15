#include "tests_common.h"

#include "repr.h"
#include "feature_world.h"

extern "C" {
// This gets called from rust.
int run_tests(){
    extra_tests();
    feature_world();

    cout << "Checks passed: " << tests_passed << ". Checks failed: " << tests_failed << "." << endl;
    return tests_failed;
}
}
