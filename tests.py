from __future__ import annotations

import os
from collections import namedtuple
from enum import Enum
from time import time


class TokenType(Enum):
    FEATURE = "Feature: "
    SCENARIO = "Scenario: "
    BACKGROUND = "Background:"
    GIVEN = "Given "
    AND = "And "
    WHEN = "When "
    THEN = "Then "
    EQUALITY = "="
    ASSIGN = "←"
    IDENTIFIER = 0
    NUMBER = 1
    PLUS = "+"
    STAR = "*"
    MINUS = "-"
    SLASH = "/"
    LEFT_PAREN = "("
    RIGHT_PAREN = ")"
    LEFT_BRACKET = "["
    RIGHT_BRACKET = "]"
    PI = "π"
    COMMA = ","
    ROOT = "√"
    PIPE = "|"
    STRING = 3
    EOF = 2
    DOT = 4
    SCENARIO_OUTLINE = "Scenario Outline: "


class Token:
    type: TokenType
    lexeme: str | float | None
    line: int

    def __init__(self, type: TokenType, lexeme: str | float | None, line: int):
        self.type = type
        self.lexeme = lexeme
        self.line = line

    def __str__(self):
        s = self.type.name
        if self.type in [TokenType.NUMBER, TokenType.IDENTIFIER, TokenType.STRING]:
            s += ": " + str(self.lexeme)
        return s


class FunctionDef:
    is_method: bool
    return_type: str
    c_name: str | None

    def __init__(self, is_method: bool, return_type: str, c_name: str | None):
        self.is_method = is_method
        self.return_type = return_type
        self.c_name = c_name


class OperatorDef:
    symbol: TokenType
    left_type: str
    right_type: str
    return_type: str
    code_template: str

    def __init__(self, symbol: TokenType, left_type: str, right_type: str, return_type: str, code_template: str):
        self.symbol = symbol
        self.left_type = left_type
        self.right_type = right_type
        self.return_type = return_type
        self.code_template = code_template


keywords = {e.value: e for e in TokenType if isinstance(e.value, str)}
functions = {
    "color": FunctionDef(False, "Colour", "Colour"),
    "point_light": FunctionDef(False, "PointLight", "PointLight"),
    "material": FunctionDef(False, "Material", "Material"),
    "ray": FunctionDef(False, "Ray", "Ray"),
    "sphere": FunctionDef(False, "Sphere", "Sphere"),
    "intersection": FunctionDef(False, "Intersection", "Intersection"),
    "point": FunctionDef(False, "Tuple", "Point"),
    "vector": FunctionDef(False, "Tuple", "Vector"),
    "tuple": FunctionDef(False, "Tuple", "Tuple"),
    "inverse": FunctionDef(True, "Matrix", "inverse"),
    "magnitude": FunctionDef(True, "float", "magnitude"),
    "normalize": FunctionDef(True, "Tuple", "normalize"),
    "dot": FunctionDef(True, "float", "dot"),
    "cross": FunctionDef(True, "Tuple", "cross"),
    "position": FunctionDef(True, "Tuple", "position"),
    "intersect": FunctionDef(True, "Intersections", "intersect"),
    "hit": FunctionDef(True, "Intersection", "hit"),
    "transform": FunctionDef(True, "Ray", "transform"),
    "set_transform": FunctionDef(True, "void", "set_transform"),
    "normal_at": FunctionDef(True, "Tuple", "normal_at"),
    "reflect": FunctionDef(True, "Tuple", "reflect"),
    "lighting": FunctionDef(True, "Colour", "lighting")
}

transformations = ["translation", "scaling", "rotation_x", "rotation_y", "rotation_z", "shearing"]
for name in transformations:
    functions[name] = FunctionDef(False, "Matrix", "Transformation::" + name)

includes = ["common.h", "Matrix.h", "Tuple.h", "Colour.h", "Canvas.h", "Ray.h"]

binary_operators = [
    OperatorDef(TokenType.STAR, "Matrix", "Matrix", "Matrix", "<a>.multiply(<b>)"),
    OperatorDef(TokenType.STAR, "Matrix", "Tuple", "Tuple", "<a>.multiply(<b>)"),
    OperatorDef(TokenType.PLUS, "Tuple", "Tuple", "Tuple", "<a>.add(<b>)"),
    OperatorDef(TokenType.MINUS, "Tuple", "Tuple", "Tuple", "<a>.subtract(<b>)"),
    OperatorDef(TokenType.STAR, "Tuple", "float", "Tuple", "<a>.scale(<b>)"),
    OperatorDef(TokenType.SLASH, "Tuple", "float", "Tuple", "<a>.divide(<b>)"),
    OperatorDef(TokenType.PLUS, "Colour", "Colour", "Colour", "<a>.add(<b>)"),
    OperatorDef(TokenType.MINUS, "Colour", "Colour", "Colour", "<a>.subtract(<b>)"),
    OperatorDef(TokenType.STAR, "Colour", "float", "Colour", "<a>.scale(<b>)"),
    OperatorDef(TokenType.STAR, "Colour", "Colour", "Colour", "<a>.multiply(<b>)")
]

for op in [TokenType.STAR, TokenType.PLUS, TokenType.SLASH, TokenType.MINUS]:
    binary_operators.append(OperatorDef(op, "float", "float", "float", "<a> " + op.value + " <b>"))

unary_operators = [
    OperatorDef(TokenType.ROOT, "", "float", "float", "sqrt(<b>)"),
    OperatorDef(TokenType.MINUS, "", "float", "float", "-<b>"),
    OperatorDef(TokenType.MINUS, "", "Tuple", "Tuple", "<b>.negate()")
]

# Could just keep line breaks as a token type, but I like the idea of insignificant whitespace.
terminators = [TokenType.EOF, TokenType.AND, TokenType.GIVEN, TokenType.THEN, TokenType.WHEN, TokenType.SCENARIO,
               TokenType.COMMA, TokenType.RIGHT_PAREN, TokenType.RIGHT_BRACKET, TokenType.SCENARIO_OUTLINE]

Expr = namedtuple("Expr", "c_code type")

variables = {
    "identity_matrix": Expr(c_code="Transformation::identity()", type="Matrix")
}

Field = namedtuple("Field", "name type is_getter is_pointer")

fields = {
    "Colour": [
        Field(name="red", type="float", is_getter=False, is_pointer=False),
        Field(name="green", type="float", is_getter=False, is_pointer=False),
        Field(name="blue", type="float", is_getter=False, is_pointer=False)
    ],
    "Ray": [
        Field(name="origin", type="Tuple", is_getter=False, is_pointer=False),
        Field(name="direction", type="Tuple", is_getter=False, is_pointer=False)
    ],
    "Intersection": [
        Field(name="t", type="float", is_getter=False, is_pointer=False),
        Field(name="object", type="Sphere", is_getter=False, is_pointer=True)
    ],
    "Tuple": [
        Field(name="x", type="float", is_getter=True, is_pointer=False),
        Field(name="y", type="float", is_getter=True, is_pointer=False),
        Field(name="z", type="float", is_getter=True, is_pointer=False),
        Field(name="w", type="float", is_getter=True, is_pointer=False)
    ],
    "Intersections": [
        Field(name="count", type="float", is_getter=True, is_pointer=False)
    ],
    "Sphere": [
        Field(name="transform", type="Matrix", is_getter=False, is_pointer=False),
        Field(name="material", type="Material", is_getter=False, is_pointer=False)
    ],
    "PointLight": [
        Field(name="intensity", type="Colour", is_getter=False, is_pointer=False),
        Field(name="position", type="Tuple", is_getter=False, is_pointer=False)
    ],
    "Material": [
        Field(name="color", type="Colour", is_getter=False, is_pointer=False),
        Field(name="ambient", type="float", is_getter=False, is_pointer=False),
        Field(name="diffuse", type="float", is_getter=False, is_pointer=False),
        Field(name="specular", type="float", is_getter=False, is_pointer=False),
        Field(name="shininess", type="float", is_getter=False, is_pointer=False)
    ]
}

getter_collections = {
    "Intersections": "Intersection"
}


def scan(src: str) -> list[Token]:
    line = 1
    start = 0
    tokens = []
    while start < len(src):
        while start < len(src) and (src[start] == " " or src[start] == "\n"):
            if src[start] == "\n":
                line += 1
            start += 1

        if start >= len(src):
            break

        matched_any_keyword = False
        for lexeme, type in keywords.items():
            match = True
            for offset in range(len(lexeme)):
                if (start + offset) >= len(src) or src[start + offset] != lexeme[offset]:
                    match = False
                    break

            if match:
                start += len(lexeme)
                tokens.append(Token(type, "", line))

                if type in [TokenType.SCENARIO, TokenType.FEATURE, TokenType.SCENARIO_OUTLINE]:
                    s = ""
                    while start < len(src) and src[start] != "\n":
                        s += src[start]
                        start += 1
                    tokens.append(Token(TokenType.STRING, s, line))

                matched_any_keyword = True
                break

        if matched_any_keyword:
            continue

        if src[start] == "#":
            while src[start] != "\n":
                start += 1

            continue

        lexeme = ""
        while start < len(src) and src[start] != " " and src[start] != "\n" and src[start] != "#":
            found = False
            for check, type in keywords.items():
                if src[start] == check:
                    found = True
                    break

            if found:
                break

            lexeme += src[start]
            start += 1

        try:
            tokens.append(Token(TokenType.NUMBER, float(lexeme), line))
        except:
            parts = lexeme.split(".")
            for p in parts:
                if p != "":
                    tokens.append(Token(TokenType.IDENTIFIER, p, line))
                tokens.append(Token(TokenType.DOT, None, line))
            tokens = tokens[:-1]

    tokens.append(Token(TokenType.EOF, None, line + 1))
    return tokens


class ParseError(Exception):
    pass


class Compiler:
    tokens: list[Token]
    i: int
    scopes: list[dict[str, str]]  # name -> type
    current_scenario: str
    output_line_count: int
    filepath: str
    background_code: list[str]

    def __init__(self, path: str, lines: int):
        self.filepath = path
        print("Parsing", path)
        with open(path, "r") as f:
            src = f.read()

        self.output_line_count = lines
        self.current_scenario = "None"
        self.code = ""
        self.tokens = scan(src)
        self.i = 0
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
            left = Expr(c_code="M_PI", type="float")
        elif self.check(TokenType.IDENTIFIER):
            name = self.advance().lexeme
            if self.match(TokenType.LEFT_PAREN):  # function call
                args = []
                while not self.match(TokenType.RIGHT_PAREN):
                    args.append(self.parse_expression(1))
                    self.match(TokenType.COMMA)

                for fname, func in functions.items():
                    if fname == name:
                        if func.is_method:
                            left = Expr(c_code="{}.{}({})".format(args[0].c_code, func.c_name,
                                                                  ", ".join([arg.c_code for arg in args[1:]])),
                                        type=func.return_type)
                        else:
                            left = Expr(c_code="{}({})".format(func.c_name, ", ".join([arg.c_code for arg in args])),
                                        type=func.return_type)
                        break
                else:
                    if name == "intersections":  # cringe special case
                        left = Expr(c_code="Intersections({" + ", ".join([arg.c_code for arg in args]) + "})",
                                    type="Intersections")
                    else:
                        self.error("Unknown function: " + name)

            else:  # variable access
                if name in variables:
                    left = variables[name]
                elif name in ["true", "false"]:
                    left = Expr(c_code=name, type="bool")
                else:
                    left = Expr(c_code=name, type=self.get_var_type(name))

        elif self.check(TokenType.NUMBER):  # float literal
            left = Expr(c_code=str(self.advance().lexeme), type="float")

        while True:
            if self.match(TokenType.DOT):
                if left is None:
                    self.error("Get property on None expression")

                obj = left.c_code
                field = str(self.consume(TokenType.IDENTIFIER, "Expect identifier after '.'").lexeme)
                obj_type = left.type

                if obj_type in fields:
                    for option in fields[obj_type]:
                        if option.name == field:
                            c_code = obj + "." + field
                            if option.is_getter:
                                c_code = c_code + "()"
                            if option.is_pointer:
                                c_code = "(*" + c_code + ")"

                            left = Expr(c_code=c_code, type=option.type)
                            break

                    else:
                        self.error("Unknown field {} on type {}".format(field, obj_type))
                else:
                    self.error("Unknown field {} on type {}".format(field, obj_type))

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
        return None

    def parse_expression(self, precedence=0, left=None) -> Expr | None:
        if left is None:
            left = self.parse_unary()

        operator = self.peek().type

        if operator in terminators:
            # TODO: just treat equality and assignment as normal ops so no special case for void function calls
            if left is not None and left.type == "void":
                self.line(left.c_code + ";")

            return left

        if operator in [TokenType.EQUALITY, TokenType.ASSIGN] and precedence > 0:
            return left

        self.advance()  # consume the operator
        right = self.parse_expression(precedence + 1)

        # These two can end the expression parsing. They don't return a value and correspond to c statement.
        if operator == TokenType.ASSIGN:
            if right.type is None:
                self.error("Cannot assign to value of unknown type: {} = {};".format(left.c_code, right.c_code))

            if "." in left.c_code:  # set field
                # TODO: type check
                self.line("{} = {};".format(left.c_code, right.c_code))
            else:  # declare variable
                self.put_var_type(left.c_code, right.type)
                self.line("{} {} = {};".format(right.type, left.c_code, right.c_code))
            return None
        elif operator == TokenType.EQUALITY:
            if left.type != right.type:
                self.error("Cannot assert equality of different types: {} and {}".format(left.type, right.type))

            if left.type == "float":
                assertion = "almostEqual({}, {})".format(left.c_code, right.c_code)
            elif left.type in ["Matrix", "Tuple", "Colour", "Sphere", "Intersection", "Material"]:
                assertion = "{}.equals({})".format(left.c_code, right.c_code)
            elif left.type in ["int"]:
                assertion = "{} == {}".format(left.c_code, right.c_code)
            else:
                self.error("Cannot assert equality of unknown type: " + str(left.type))
                assertion = "false"

            self.line("_scenarioPassed = _scenarioPassed && ({});".format(assertion))
            return None

        # Check all the binary operators that have a result.
        # If it matches one, evaluate that template and then parse a new expression with that as the left side.
        if left is not None and right is not None:
            for option in binary_operators:
                if operator == option.symbol and left.type == option.left_type and right.type == option.right_type:
                    code = option.code_template.replace("<a>", left.c_code).replace("<b>", right.c_code)
                    expr = Expr(c_code=code, type=option.return_type)
                    return self.parse_expression(precedence=precedence, left=expr)

        self.error("Expect expression")

    def read_name(self) -> str:
        return self.consume(TokenType.STRING, "Expect string.").lexeme

    def match(self, type: TokenType) -> bool:
        if self.check(type):
            self.i += 1
            return True
        return False

    def check(self, type: TokenType) -> bool:
        return self.peek().type == type

    def peek(self) -> Token:
        return self.tokens[self.i]

    def advance(self) -> Token:
        self.i += 1
        return self.tokens[self.i - 1]

    def consume(self, type: TokenType, err: str) -> Token:
        if not self.match(type):
            self.i += 1
            self.error(err)

        return self.tokens[self.i - 1]

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

    def error(self, err):
        line_num = self.tokens[self.i - 1].line
        print("Error on line {} ({}).\n    - {}".format(line_num, self.current_scenario, err))
        # Inefficient but only on error so who cares
        this_line = [str(t) for t in self.tokens if t.line == line_num]
        print("    - " + str(this_line))
        raise ParseError()


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

    for root, dirs, files in os.walk("tests"):
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

    with open("src/tests.cc", "w") as f:
        f.write(run_tests)

    build_start_time = time()
    print("=" * 30)
    os.system("/Applications/CLion.app/Contents/bin/cmake/mac/bin/cmake -DCMAKE_BUILD_TYPE=Debug -DCMAKE_MAKE_PROGRAM=/Applications/CLion.app/Contents/bin/ninja/mac/ninja -G Ninja -S /Users/luke/Documents/mods/raytracer -B /Users/luke/Documents/mods/raytracer/cmake-build-debug")
    os.system("/Applications/CLion.app/Contents/bin/cmake/mac/bin/cmake --build /Users/luke/Documents/mods/raytracer/cmake-build-debug --target raytracer_tests -j 6")
    print("=" * 30)
    run_start_time = time()
    os.system("/Users/luke/Documents/mods/raytracer/cmake-build-debug/raytracer_tests")
    run_end_time = time()
    print("- Build: " + str(int(round(run_start_time - build_start_time, 3) * 1000)) + " ms.")
    print("- Parse: " + str(int(round(build_start_time - parse_start_time, 3) * 1000)) + " ms.")
