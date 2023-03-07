import os
from time import time


def test_header_parser():
    from HeaderParser import HeaderParser
    from AST import ClassPrototype, FunctionPrototype, FieldPrototype

    def build_expected() -> dict[str, ClassPrototype]:
        results: dict[str, ClassPrototype] = {
            "Example": ClassPrototype(name="Example", filename="example.h"),
            "Another": ClassPrototype(name="Another", filename="example.h", is_abstract=True),
            "YetAnother": ClassPrototype(name="YetAnother", extends="Another", filename="example.h")
        }

        results["Example"].methods = [
            FunctionPrototype(name="create", return_type="Another", is_static=True, namespace="Example")
        ]
        results["Example"].constructors = [
            FunctionPrototype(name="Example", return_type="Example", is_static=True, argument_types=["bool", "float"]),
            FunctionPrototype(name="Example", return_type="Example", is_static=True, argument_types=["int", "double"]),
            FunctionPrototype(name="Example", return_type="Example", is_static=True)
        ]

        results["Another"].methods = [
            FunctionPrototype(name="add", return_type="float", is_static=False, argument_types=["int", "double"]),
            FunctionPrototype(name="getSomething", return_type="bool*", is_static=False, argument_types=["Example"]),
            FunctionPrototype(name="doSomething", return_type="int", is_static=False),
            FunctionPrototype(name="look", return_type="int", is_static=False, argument_types=["int*", "int**", "int"])
        ]
        results["Another"].constructors = [
            FunctionPrototype(name="Another", return_type="Another", is_static=True, argument_types=["int"]),
            FunctionPrototype(name="Another", return_type="Another", is_static=True, argument_types=["double"]),
        ]
        results["Another"].fields = [
            FieldPrototype(name="y", type="double", is_static=False)
        ]

        results["YetAnother"].methods = [
            FunctionPrototype(name="getSomething", return_type="bool*", is_static=False, argument_types=["Example"])
        ]
        results["YetAnother"].constructors = [
            FunctionPrototype(name="YetAnother", return_type="YetAnother", is_static=True,
                              argument_types=["YetAnother"])
        ]
        results["YetAnother"].fields = [
            FieldPrototype(name="something", type="bool", is_static=False)
        ]

        return results

    expected_example_classes = build_expected()
    tests_passed = [True]

    def err(msg):
        print(msg)
        tests_passed[0] = False

    def assert_equal_str(expected, found, msg, class_name):
        if str(expected) != str(found):
            if not has_failed[0]:
                err("FAIL: " + class_name)
            err("  " + msg)
            err("    Expected: " + str(expected))
            err("       Found: " + str(found))

            has_failed[0] = True

    def compare(expected_entries, found_entries, title, class_name):
        for i, expect in enumerate(expected_entries):
            if len(found_entries) <= i:
                assert_equal_str(expect, None, "Missing " + title, class_name)
            else:
                assert_equal_str(expect, found_entries[i], "Mismatched " + title, class_name)

    print("TESTING HEADER PARSER")
    actual_example_classes_raw = HeaderParser("tests/example.h").parse()
    actual_example_classes: dict[str, ClassPrototype] = {obj.name: obj for obj in actual_example_classes_raw}
    if len(actual_example_classes) != len(actual_example_classes):
        err("FAIL: duplicate class names in " + str([c.name for c in actual_example_classes_raw]))

    for name, expected in expected_example_classes.items():
        if name not in actual_example_classes:
            err("Expected class named '{}'.".format(name))
            continue

        has_failed = [False]
        found = actual_example_classes[name]
        compare(expected.fields, found.fields, "Field", name)
        compare(expected.constructors, found.constructors, "Constructor", name)
        compare(expected.methods, found.methods, "Method", name)

    for name, found in actual_example_classes.items():
        if name not in expected_example_classes:
            err("Found unexpected class named '{}'".format(name))

    if tests_passed[0]:
        print("PASS: test_header_parser")
    else:
        print("FAIL: test_header_parser")


def exec_tests_cc():
    build_start_time = time()
    print("=" * 30)
    os.system(
        "/Applications/CLion.app/Contents/bin/cmake/mac/bin/cmake -DCMAKE_BUILD_TYPE=Debug -DCMAKE_MAKE_PROGRAM=/Applications/CLion.app/Contents/bin/ninja/mac/ninja -G Ninja -S . -B cmake-build-debug")
    os.system(
        "/Applications/CLion.app/Contents/bin/cmake/mac/bin/cmake --build cmake-build-debug --target raytracer_tests -j 6")
    print("=" * 30)
    run_start_time = time()
    os.system("cmake-build-debug/raytracer_tests")
    print("- CMake Build: " + str(int(round(run_start_time - build_start_time, 3) * 1000)) + " ms.")


def test_gherkin_parser():
    from GherkinParser import GherkinParser, find_feature_files, parse_feature_files, cpp_classes
    from CodeGen import CodeGen

    features = parse_feature_files(["tests/example.feature"])
    [[[print(stmt) for stmt in s.statements] for s in f.scenarios] for f in features]
    CodeGen(features, "src/tests.cc", list({x.filename.replace("src/", "") for x in cpp_classes.values()})).build()
    exec_tests_cc()


def test_ray_tracer():
    from GherkinParser import GherkinParser, find_feature_files, parse_feature_files, cpp_classes
    from CodeGen import CodeGen

    features = parse_feature_files(find_feature_files("tests/book"))
    CodeGen(features, "src/tests.cc", list({x.filename.replace("src/", "") for x in cpp_classes.values()})).build()
    exec_tests_cc()


if __name__ == "__main__":
    test_header_parser()
    print("=" * 30)
    test_gherkin_parser()
    print("=" * 30)
    test_ray_tracer()
