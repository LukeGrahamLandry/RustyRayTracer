#include "tests_common.h"

void shader_types_struct_repr_matches(){
#define ASSERT_REPR(i, name) \
    start_test("Rust and C++ have same repr for type " #name); \
    assert_eq(rust_struct_sizes[i], sizeof(name)); \
    assert_eq(rust_struct_aligns[i], alignof(name)); \

    usize count = 8;
    usize rust_struct_sizes[count];
    usize rust_struct_aligns[count];
    get_structs_repr(count, rust_struct_sizes, rust_struct_aligns);

    ASSERT_REPR(0, Camera);
    ASSERT_REPR(1, Material);
    ASSERT_REPR(2, PointLight);
    ASSERT_REPR(3, ShapeType);
    ASSERT_REPR(4, Shape);
    ASSERT_REPR(5, ShaderInputs);
    ASSERT_REPR(6, World);
    ASSERT_REPR(7, float4x4);

#undef ASSERT_REPR
}

void extra_tests(){
    shader_types_struct_repr_matches();
}
