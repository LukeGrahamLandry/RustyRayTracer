from __future__ import annotations

import os
from collections import namedtuple
from enum import Enum


class TokenType(Enum):
    FEATURE = "Feature: "
    SCENARIO = "Scenario: "
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
    EOF = 2


class Token:
    type: TokenType
    lexeme: str | float | None

    def __init__(self, type: TokenType, lexeme: str | float | None):
        self.type = type
        self.lexeme = lexeme

    def __str__(self):
        s = self.type.name
        if self.type == TokenType.NUMBER or self.type == TokenType.IDENTIFIER:
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
    "point": FunctionDef(False, "Tuple", "Point"),
    "vector": FunctionDef(False, "Tuple", "Vector"),
    "tuple": FunctionDef(False, "Tuple", "Tuple"),
    "inverse": FunctionDef(True, "Matrix", None)
}

transformations = ["translation", "scaling", "rotation_x", "rotation_y", "rotation_z", "shearing"]
for name in transformations:
    functions[name] = FunctionDef(False, "Matrix", "Transformation::" + name)

variables = {
    "identity_matrix": "Transformation::identity()"
}

includes = ["common.h", "Matrix.h", "Tuple.h", "Colour.h", "Canvas.h"]

binary_operators = [
    OperatorDef(TokenType.STAR, "Matrix", "Matrix", "Matrix", "<a>.multiply(<b>)")
]

for op in [TokenType.STAR, TokenType.PLUS, TokenType.SLASH, TokenType.MINUS]:
    binary_operators.append(OperatorDef(op, "float", "float", "float", "<a> " + op.value + " <b>"))

unary_operators = [
    OperatorDef(TokenType.ROOT, "", "float", "float", "sqrt(<b>)")
]

# Could just keep line breaks as a token type, but I like the idea of insignificant whitespace.
terminators = [TokenType.EOF, TokenType.AND, TokenType.GIVEN, TokenType.THEN, TokenType.WHEN, TokenType.SCENARIO]

Expr = namedtuple("Expr", "c_code type")


def scan(src: str) -> list[Token]:
    start = 0
    tokens = []
    while start < len(src):
        while start < len(src) and (src[start] == " " or src[start] == "\n"):
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
                tokens.append(Token(type, ""))
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
            tokens.append(Token(TokenType.NUMBER, float(lexeme)))
        except:
            tokens.append(Token(TokenType.IDENTIFIER, lexeme))

    tokens.append(Token(TokenType.EOF, None))
    return tokens


class Compiler:
    tokens: list[Token]
    i: int
    scopes: list[dict[str, str]]  # name -> type
    panic: bool

    def __init__(self, src: str):
        self.panic = False
        self.code = ""
        self.tokens = scan(src)
        self.i = 0
        self.scopes = []

    def build(self) -> str:
        self.push_scope()
        self.line("int _passedScenarioCount = 0;")
        self.consume(TokenType.FEATURE, "Expect 'Feature' at beginning of file.")

        name = self.read_name()
        self.line("cout << \"FEATURE: " + name + ".\" << endl;")
        count = 0
        while not self.match(TokenType.EOF):
            self.parse_scenario()
            count += 1

        self.line('cout << "{0} passed " << _passedScenarioCount << " of {1} tests." << endl;\n'.format(name, count))
        self.pop_scope()
        return self.code

    def parse_scenario(self):
        self.consume(TokenType.SCENARIO, "Expect 'Scenario'.")
        name = self.read_name()
        self.push_scope()
        self.line("bool _scenarioPassed = true;")

        self.consume(TokenType.GIVEN, "Expect 'Given' as first statement.")
        self.parse_statement()
        self.consume(TokenType.THEN, "Expect 'Then' as first statement.")
        self.parse_statement()

        self.line("if (_scenarioPassed){")
        self.line('    cout << " - PASS: " << "{}" << endl;'.format(name))
        self.line("    _passedScenarioCount++;")
        self.line("} else {")
        self.line('    cout << " - FAIL: " << "{}" << endl;'.format(name))
        self.line("}")
        self.pop_scope()

    def parse_statement(self):
        self.parse_expression()
        while self.match(TokenType.AND):
            self.parse_expression()

    def parse_expression(self, precedence=0, left=None) -> Expr | None:
        if self.check(TokenType.PI):
            left = Expr(c_code="M_PI", type="float")
        elif self.check(TokenType.IDENTIFIER):
            name = self.advance().lexeme
            if self.match(TokenType.LEFT_PAREN):  # function call
                pass
            else:  # variable access
                left = Expr(c_code=name, type=self.get_var_type(name))

        elif self.check(TokenType.NUMBER):  # float literal
            left = Expr(c_code=self.advance().lexeme, type="float")

        operator = self.peek().type

        if operator in terminators:
            return left

        if operator in [TokenType.EQUALITY, TokenType.ASSIGN] and precedence > 0:
            return left

        self.advance()  # consume the operator

        right = self.parse_expression(precedence + 1)
        print(left, operator, right, precedence)

        if left is None:
            for option in unary_operators:
                if operator == option.symbol and right.type == option.right_type:
                    code = option.code_template.replace("<b>", right.c_code)
                    expr = Expr(c_code=code, type=option.return_type)
                    return self.parse_expression(precedence=precedence, left=expr)

        # These two can end the expression parsing. They don't return a value and correspond to c statement.
        if operator == TokenType.ASSIGN:
            self.put_var_type(left.c_code, right.type)
            self.line("{} {} = {};".format(right.type, left.c_code, right.c_code))
            return None
        elif operator == TokenType.EQUALITY:
            if left.type != right.type:
                self.error("Cannot assert unequal types.")

            assertion = "false"
            if left.type == "float":
                assertion = "almostEqual({}, {})".format(left.c_code, right.c_code)

            self.line("_scenarioPassed = _scenarioPassed && ({});".format(assertion))
            return None

        # Check all the binary operators that have a result.
        # If it matches one, evaluate that template and then parse a new expression with that as the left side.
        for option in binary_operators:
            if operator == option.symbol and left.type == option.left_type and right.type == option.right_type:
                code = option.code_template.replace("<a>", left.c_code).replace("<b>", right.c_code)
                expr = Expr(c_code=code, type=option.return_type)
                return self.parse_expression(precedence=precedence, left=expr)

        self.error("Expect expression")

    def read_name(self) -> str:
        s = ""
        while self.match(TokenType.IDENTIFIER):
            s += self.tokens[self.i - 1].lexeme + " "
        return s[:-1]

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
            self.error(err)

        return self.tokens[self.i - 1]

    def line(self, c: str):
        self.code += ("    " * (len(self.scopes) + 1)) + c + "\n"

    @staticmethod
    def compile(filepath: str) -> str:
        with open(filepath, "r") as f:
            src = f.read()

        print("Compiling", filepath)
        return Compiler(src).build()

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
        print(err)
        self.panic = True


if __name__ == "__main__":
    run_tests = ""
    for file in includes:
        run_tests += "#include \"" + file + "\"\n"

    run_tests += "\n\nint main(){\n"
    for root, dirs, files in os.walk("tests"):
        for name in files:
            if name != "thing.feature":
                continue
            path = os.path.join(root, name)
            run_tests += Compiler.compile(path)

        break

    run_tests += "\n    return 0;\n}\n"

    with open("src/tests.cc", "w") as f:
        f.write(run_tests)
