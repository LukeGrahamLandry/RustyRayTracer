from __future__ import annotations

import os
from collections import namedtuple
from enum import Enum
from time import time

from config import *
from common import *
from header import *

cpp_classes: dict[str, ClassPrototype] = {obj.name: obj for obj in walk_headers("../src")}
for klass in cpp_classes.values():
    if klass.extends is not None:
        extends = cpp_classes[klass.extends]
        for f in extends.fields:
            klass.fields.append(f)
        for m in extends.methods:
            klass.methods.append(m)
cpp_classes["Color"] = cpp_classes["Colour"]
cpp_classes["Vector"].constructors[0].return_type = "Tuple"
cpp_classes["Point"].constructors[0].return_type = "Tuple"
cpp_classes["Plane"].constructors.append(FunctionPrototype(name="Plane", return_type="Plane", is_static=True))
cpp_classes["Sphere"].constructors.append(FunctionPrototype(name="Sphere", return_type="Sphere", is_static=True))
with open("parsed_headers.txt", "w") as f:
    [f.write(str(s) + "\n\n") for s in cpp_classes.values()]


class Compiler(AbstractParser):
    scopes: list[dict[str, str]]  # name -> type
    current_scenario: str | None
    output_line_count: int
    background_code: list[str]

    def __init__(self, path: str, lines: int):
        super().__init__(path, gherkin_keywords)

        self.output_line_count = lines
        self.current_scenario = None
        self.code = ""
        self.scopes = []
        self.background_code = []

        self.build()

    def build(self):
        self.push_scope()
        self.line("int _passedScenarioCount = 0;")

        self.consume(TokenType.FEATURE, "Expect 'Feature' at beginning of file.")
        name = self.read_name()
        self.line("cout << \"FEATURE: " + name + ".\" << endl;")

        self.setup_background()

        count = 0
        while not self.match(TokenType.EOF):
            self.parse_scenario()
            count += 1

        self.line('cout << "{} passed " << _passedScenarioCount << " of {} tests." << endl;'.format(name, count))
        self.pop_scope()

    def parse_scenario(self):
        start = len(self.code)
        line_count = self.output_line_count

        self.current_scenario = "Untitled on Line " + str(self.peek().line)
        try:
            self.consume(TokenType.SCENARIO, "Expect 'Scenario'.")
            self.current_scenario = self.read_name()
            self.push_scope()
            self.line("bool _scenarioPassed = true;")

            # Inject background setup for every test. Not at top level so variables reset for each scenario.
            # Used by materials.feature
            for line in self.background_code:
                self.line(line.strip())
            # Start a new scope which lets scenarios redefine names from the background setup.
            self.push_scope()

            self.consume(TokenType.GIVEN, "Expect 'Given' as first statement.")
            self.parse_statement()

            while self.match(TokenType.WHEN):
                # self.push_scope()
                self.parse_statement()
                self.consume(TokenType.THEN, "Expect 'Then' following 'When'.")
                self.parse_statement()
                # self.pop_scope()

            if self.match(TokenType.THEN):
                self.parse_statement()

            self.line("if (_scenarioPassed){")
            self.line('    cout << " - PASS: {}" << endl;'.format(self.current_scenario))
            self.line("    _passedScenarioCount++;")
            self.line("} else {")
            self.line('    cout << " - FAIL: {}" << endl;'.format(self.current_scenario))
            self.line('    cout << "         at src/tests.cc:{}" << endl;'.format(line_count))
            self.line("}")
            self.pop_scope()
            self.pop_scope()
        except ParseError:
            self.code = self.code[:start]
            self.output_line_count = line_count
            while not self.check(TokenType.SCENARIO) and not self.check(TokenType.EOF):
                self.i += 1
            self.scopes = self.scopes[:1]
            self.line('cout << " - ERROR: {}" << endl;'.format(self.current_scenario))

    def setup_background(self):
        if self.match(TokenType.BACKGROUND):
            start = len(self.code)
            line_count = self.output_line_count

            # Note: No inner compiler scope so the types get saved in the outermost one.
            #       Which works out to the behaviour I want.
            self.consume(TokenType.GIVEN, "Expect 'Given' as first statement.")
            self.parse_statement()

            self.background_code += self.code[start:].split("\n")
            self.code = self.code[:start]
            self.output_line_count = line_count

    def parse_statement(self):
        self.parse_expression()
        while self.match(TokenType.AND):
            self.parse_expression()

    def parse_primary(self):
        left = None
        if self.match(TokenType.PI):
            left = Expr(c_code="M_PI", type="double")
        elif self.check(TokenType.IDENTIFIER):
            name = self.advance().lexeme
            if self.match(TokenType.LEFT_PAREN):  # function call
                args = []
                while not self.match(TokenType.RIGHT_PAREN):
                    args.append(self.parse_expression(1))
                    self.match(TokenType.COMMA)

                left = self.create_function_call(name, args)

            else:  # variable access
                if name in variables:
                    left = variables[name]
                elif name in ["true", "false"]:
                    left = Expr(c_code=name, type="bool")
                else:
                    left = Expr(c_code=name, type=self.get_var_type(name))

        elif self.check(TokenType.NUMBER):  # double literal
            left = Expr(c_code=str(self.advance().lexeme), type="double")

        while True:
            if self.match(TokenType.DOT):
                field_name = str(self.consume(TokenType.IDENTIFIER, "Expect identifier after '.'").lexeme)
                left = self.create_field_access(field_name, left)

            elif self.match(TokenType.LEFT_BRACKET):
                if left is None:
                    self.error("Get index on None expression")

                index = self.parse_expression()
                self.consume(TokenType.RIGHT_BRACKET, "Expect closing ']'.")
                for collection_type, value_type in getter_collections.items():
                    if left.type == collection_type:
                        left = Expr(c_code=left.c_code + ".get(" + index.c_code + ")", type=value_type)
                        break
                else:
                    self.error("Invalid collection type: " + left.type)

            else:
                break

        return left

    def parse_unary(self):
        right = self.parse_primary()
        if right is not None:
            return right

        operator = self.advance().type
        right = self.parse_unary()
        for option in unary_operators:
            if operator == option.symbol and right.type == option.right_type:
                code = option.code_template.replace("<b>", right.c_code)
                return Expr(c_code=code, type=option.return_type)

        self.error("Invalid unary operator {} on type {} ".format(operator.name, right.type))

    def parse_expression(self, precedence=0, left=None) -> Expr | None:
        if left is None:
            left = self.parse_unary()

        operator = self.peek().type

        if operator in terminators:
            # TODO: just treat equality and assignment as normal ops so no special case for void function calls
            if left is not None and left.type == "void":
                self.line(left.c_code + ";")

            if left is not None and left.type == "bool" and precedence == 0:
                self.line("_scenarioPassed = _scenarioPassed && ({});".format(left.c_code))

            return left

        if operator in [TokenType.EQUALITY, TokenType.ASSIGN] and precedence > 0:
            return left

        self.advance()  # consume the operator
        right = self.parse_expression(precedence + 1)

        # These two can end the expression parsing. They don't return a value and correspond to c statement.
        if operator == TokenType.ASSIGN:
            if right.type is None:
                self.error("Cannot assign to value of unknown type: {} = {};".format(left.c_code, right.c_code))

            right = right.match_pointer_indirection(left)

            if "." in left.c_code:  # set field
                # TODO: type check
                self.line("{} = {};".format(left.c_code, right.c_code))
            else:  # declare variable
                self.put_var_type(left.c_code, right.type)
                self.line("{} {} = {};".format(right.type, left.c_code, right.c_code))
            return Expr("", "void")
        elif operator == TokenType.EQUALITY:
            left = left.dereference()
            right = right.dereference()

            # cringe: it should know about abstract classes
            # if left.type != right.type:
            #    self.error("Cannot assert equality of different types: {} and {}".format(left.type, right.type))

            if left.type in cpp_classes:
                equality = cpp_classes[left.type].get_methods("equals")
                if len(equality) > 0:
                    assertion = "{}.equals({})".format(left.c_code, right.c_code)
            elif left.type == "double":
                assertion = "almostEqual({}, {})".format(left.c_code, right.c_code)
            elif left.type in ["int", "bool"]:
                assertion = "{} == {}".format(left.c_code, right.c_code)
            else:
                self.error("Cannot assert equality of unknown type: " + str(left.type))
                assertion = "false"

            self.line("_scenarioPassed = _scenarioPassed && ({});".format(assertion))
            return Expr("", "void")

        # Check all the binary operators that have a result.
        # If it matches one, evaluate that template and then parse a new expression with that as the left side.
        if left is not None and right is not None:
            for option in binary_operators:
                if operator == option.symbol and left.type == option.left_type and right.type == option.right_type:
                    code = option.code_template.replace("<a>", left.c_code).replace("<b>", right.c_code)
                    expr = Expr(c_code=code, type=option.return_type)
                    return self.parse_expression(precedence=precedence, left=expr)

        print(left, operator, right)
        self.error("Expect expression")

    def create_function_call(self, spec_name: str, args: list[Expr]) -> Expr:
        # Check as constructor
        as_class_name = spec_name.replace("_", " ").title().replace(" ", "")
        if as_class_name in cpp_classes:
            klass = cpp_classes[as_class_name]
            for func in klass.constructors:
                if func.match(args):
                    return func.create_call(args)

        # Check as method
        if len(args) > 0:
            as_class_name = args[0].type.replace("_", " ").title().replace(" ", "")
            if as_class_name in cpp_classes:
                klass = cpp_classes[as_class_name]
                options = klass.get_methods(spec_name)
                for func in options:
                    if func.match(args[1:]):
                        return func.create_call(args)

        # Check as any static
        for klass in cpp_classes.values():
            for func in klass.get_methods(spec_name):
                print(func)
                if func.is_static and func.match(args):
                    return func.create_call(args)

        self.error("Undefined function: " + spec_name + " with args " + str([str(a) for a in args]))

    def create_field_access(self, spec_name: str, object: Expr) -> Expr:
        object = object.dereference()
        if object.type not in cpp_classes:
            self.error("Unrecognised type in: " + str(object))

        klass = cpp_classes[object.type]

        # Check as field
        if spec_name in klass.get_fields():
            field = klass.get_fields()[spec_name]
            return Expr(c_code=object.c_code + "." + field.name, type=field.type)

        # Check as getter
        for func in klass.get_methods(spec_name):
            if not func.is_static and func.match([]):
                return func.create_call([object])

        self.error("Undefined field: " + spec_name + " on " + object.type)

    def line(self, c: str):
        self.code += ("    " * (len(self.scopes) + 1)) + c + "\n"
        self.output_line_count += 1

    def get_var_type(self, name: str) -> str:
        for i in range(len(self.scopes)):
            data = self.scopes[len(self.scopes) - i - 1]
            if name in data:
                return data[name]

    def put_var_type(self, name: str, type: str):
        self.scopes[len(self.scopes) - 1][name] = type

    def push_scope(self):
        self.line("{")
        self.scopes.append({})

    def pop_scope(self):
        del self.scopes[len(self.scopes) - 1]
        self.line("}")

    def get_err_context(self):
        if self.current_scenario is None:
            return super().get_err_context()
        else:
            return self.current_scenario


if __name__ == "__main__":
    parse_start_time = time()
    run_tests = "#include <chrono>\n"
    for file in includes:
        run_tests += "#include \"" + file + "\"\n"

    run_tests += "\n// THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT MANUALLY. "
    run_tests += "\n\nint main(){\n"
    run_tests += "    long _start_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();\n"
    line_count = len(run_tests.splitlines()) + 1
    test_count = 0

    for root, dirs, files in os.walk("../tests"):
        for name in files:
            path = os.path.join(root, name)

            c = Compiler(path, line_count)
            run_tests += c.code
            line_count = c.output_line_count
            test_count += 1

    run_tests += "    long _end_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();\n"
    run_tests += '    cout << "' + ("=" * 30) + '" << endl;\n'
    run_tests += '    cout << "- Execute: " << (_end_time - _start_time) << " ms." << endl;\n'
    run_tests += "    return 0;\n}\n"

    with open("../src/tests.cc", "w") as f:
        f.write(run_tests)

    build_start_time = time()
    print("=" * 30)
    os.system(
        "/Applications/CLion.app/Contents/bin/cmake/mac/bin/cmake -DCMAKE_BUILD_TYPE=Debug -DCMAKE_MAKE_PROGRAM=/Applications/CLion.app/Contents/bin/ninja/mac/ninja -G Ninja -S /Users/luke/Documents/mods/raytracer -B /Users/luke/Documents/mods/raytracer/cmake-build-debug")
    os.system(
        "/Applications/CLion.app/Contents/bin/cmake/mac/bin/cmake --build /Users/luke/Documents/mods/raytracer/cmake-build-debug --target raytracer_tests -j 6")
    print("=" * 30)
    run_start_time = time()
    os.system("/Users/luke/Documents/mods/raytracer/cmake-build-debug/raytracer_tests")
    run_end_time = time()
    print("- Build: " + str(int(round(run_start_time - build_start_time, 3) * 1000)) + " ms.")
    print("- Parse: " + str(int(round(build_start_time - parse_start_time, 3) * 1000)) + " ms.")
