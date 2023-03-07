from __future__ import annotations

import os
import AST
from HeaderParser import *

cpp_classes: dict[str, ClassPrototype] = {obj.name: obj for obj in walk_headers("src")}

# cringe: my header parser doesn't know how inheritance works
for klass in cpp_classes.values():
    if klass.extends is not None:
        extends = cpp_classes[klass.extends]
        for f in extends.fields:
            klass.fields.append(f)
        for m in extends.methods:
            klass.methods.append(m)
cpp_classes["Vector"].constructors[0].return_type = "Tuple"
cpp_classes["Point"].constructors[0].return_type = "Tuple"
cpp_classes["Plane"].constructors.append(FunctionPrototype(name="Plane", return_type="Plane", is_static=True))
cpp_classes["Sphere"].constructors.append(FunctionPrototype(name="Sphere", return_type="Sphere", is_static=True))

# cringe: americans
cpp_classes["Color"] = cpp_classes["Colour"]

# cringe: my header parser doesn't know about global functions
doubleAlmostEqual = FunctionPrototype(name="almostEqual", is_static=True, return_type="bool")


gherkin_keywords = dict({e.value: e for e in [
    TokenType.FEATURE,
    TokenType.SCENARIO,
    TokenType.BACKGROUND,
    TokenType.GIVEN,
    TokenType.AND,
    TokenType.WHEN,
    TokenType.THEN,
    TokenType.EQUALITY,
    TokenType.ASSIGN,
    TokenType.PLUS,
    TokenType.STAR,
    TokenType.BANG,
    TokenType.MINUS,
    TokenType.SLASH,
    TokenType.LEFT_PAREN,
    TokenType.RIGHT_PAREN,
    TokenType.LEFT_BRACKET,
    TokenType.RIGHT_BRACKET,
    TokenType.PI,
    TokenType.COMMA,
    TokenType.ROOT,
    TokenType.PIPE,
    TokenType.SCENARIO_OUTLINE
]})

# Could just keep line breaks as a token type, but I like the idea of insignificant whitespace.
terminators = [TokenType.EOF, TokenType.AND, TokenType.GIVEN, TokenType.THEN, TokenType.WHEN, TokenType.SCENARIO,
               TokenType.COMMA, TokenType.RIGHT_PAREN, TokenType.RIGHT_BRACKET, TokenType.SCENARIO_OUTLINE]


class GherkinParser(AbstractParser):
    scopes: list[dict[str, str]]  # name -> type
    current_scenario: str | None
    output_line_count: int
    background_code: list[AST.Statement]
    scenarios: list[AST.Scenario | AST.ReportErr]

    def __init__(self, path: str):
        super().__init__(path, gherkin_keywords)

        self.current_scenario = None
        self.scopes = []
        self.background_code = []
        self.scenarios = []

    def build(self) -> AST.Feature:
        self.push_scope()
        self.consume(TokenType.FEATURE, "Expect 'Feature' at beginning of file.")
        name = self.read_name()
        self.setup_background()

        while not self.match(TokenType.EOF):
            self.parse_scenario()

        self.pop_scope()

        return AST.Feature(name=name, scenarios=self.scenarios)

    def parse_scenario(self):
        self.current_scenario = "Untitled on Line " + str(self.peek().line)
        self.scenarios.append(AST.Scenario(name=self.current_scenario, statements=[], background=self.background_code))
        try:
            self.consume(TokenType.SCENARIO, "Expect 'Scenario'.")
            self.current_scenario = self.read_name()
            self.scenarios[-1].name = self.current_scenario
            self.push_scope()

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

            self.pop_scope()
            self.pop_scope()
        except ParseError:
            while not self.check(TokenType.SCENARIO) and not self.check(TokenType.EOF):
                self.i += 1
            self.scopes = self.scopes[:1]
            self.scenarios[-1] = AST.ReportErr(msg=self.current_scenario)

    def setup_background(self):
        if self.match(TokenType.BACKGROUND):
            # Note: No inner compiler scope so the types get saved in the outermost one.
            #       Which works out to the behaviour I want.
            self.consume(TokenType.GIVEN, "Expect 'Given' as first statement.")
            self.parse_statement(to_background=True)

    def parse_statement(self, to_background=False):
        stmts = [self.parse_expression()]
        while self.match(TokenType.AND):
            stmts.append(self.parse_expression())

        if to_background:
            self.background_code.extend(stmts)
        else:
            self.scenarios[-1].statements.extend(stmts)

    def parse_primary(self) -> AST.Expression | None:
        left: AST.Expression | None = None
        if self.match(TokenType.PI):
            left = AST.LiteralExpr(symbol="M_PI", type="double")
        elif self.check(TokenType.IDENTIFIER):
            name = self.advance().lexeme
            if self.match(TokenType.LEFT_PAREN):  # function call
                args = self.parse_arg_list(TokenType.RIGHT_PAREN)
                left = self.create_function_call(name, args)

            else:  # variable access
                # TODO: identity_matrix
                if name in ["true", "false"]:
                    left = AST.LiteralExpr(symbol=name, type="bool")
                else:
                    left = AST.VarAccess(name=name, type=self.get_var_type(name))

        elif self.check(TokenType.NUMBER):  # double literal
            left = AST.LiteralExpr(symbol=str(self.advance().lexeme), type="double")

        while True:
            if self.match(TokenType.DOT):
                field_name = str(self.consume(TokenType.IDENTIFIER, "Expect identifier after '.'").lexeme)
                left = self.create_field_access(field_name, left)

            elif self.match(TokenType.LEFT_BRACKET):
                if left is None:
                    self.error("Get index on None expression")

                index = self.parse_arg_list(TokenType.RIGHT_BRACKET)
                left = self.create_function_call("get", [left, *index])
            else:
                break

        return left

    def parse_unary(self) -> AST.Expression:
        right = self.parse_primary()
        if right is not None:
            return right

        operator = self.advance().type
        right = self.parse_unary()

        if operator == TokenType.MINUS and right.type == "double":
            return AST.UnaryExpr(symbol="-", value=right, type="double")
        if operator == TokenType.ROOT and right.type == "double":
            return AST.FunctionCall(func=FunctionPrototype.SQRT, args=[right], type="double")
        elif operator == TokenType.BANG and right.type == "bool":
            return AST.UnaryExpr(symbol="!", value=right, type="bool")
        elif operator == TokenType.MINUS and right.type in cpp_classes:
            return self.create_function_call("negate", [right])
        else:
            self.error("Invalid unary operator {} on type {} ".format(operator.name, right.type))

    def parse_expression(self, precedence=0, left: Optional[AST.Expression] = None) -> Optional[AST.Expression | AST.Statement]:
        if left is None:
            left = self.parse_unary()

        operator = self.peek().type

        if operator in terminators:
            # Could just treat equality and assignment as normal ops so no special case for void function calls
            if left is not None and left.type == "void":
                return AST.ExpressionStmt(value=left)

            if left is not None and left.type == "bool" and precedence == 0:
                return AST.Assertion(value=left)

            return left

        if operator in [TokenType.EQUALITY, TokenType.ASSIGN] and precedence > 0:
            return left

        self.advance()  # consume the operator
        right = self.parse_expression(precedence + 1)

        # These two can end the expression parsing. They don't return a value and correspond to c statement.
        if operator == TokenType.ASSIGN:
            if not isinstance(left, AST.VarAccess) and not isinstance(left, AST.FieldAccess):
                self.error("Cannot only assign to var or field: {} = {};".format(str(left), str(right)))

            is_declare = isinstance(left, AST.VarAccess) and self.get_var_type(left.name) is None
            right = right.match_pointer_indirection(left)
            if is_declare:
                self.put_var_type(left.name, right.type)
                return AST.VarDeclare(variable=left, value=right, type=right.type)
            else:
                return AST.Setter(variable=left, value=right)

        elif operator == TokenType.EQUALITY:
            left = left.dereference_all()
            right = right.dereference_all()

            if left.type in cpp_classes:
                return AST.Assertion(value=self.create_function_call("equals", [left, right]))
            elif left.type == "double" and right.type == "double":
                return AST.Assertion(value=AST.FunctionCall(func=doubleAlmostEqual, args=[right, left], type=doubleAlmostEqual.return_type))
            elif left.type == "bool" and right.type == "bool":
                return AST.Assertion(value=AST.BinaryExpr(symbol="==", left=left, right=right, type="bool"))
            else:
                self.error("Cannot assert equality of unknown type: {} == {}".format(left, right))

        # Check all the binary operators that have a result.
        # If it matches one, evaluate that template and then parse a new expression with that as the left side.
        if left is not None and right is not None:
            if left.type == "double" and right.type == "double":
                if operator in [TokenType.PLUS, TokenType.MINUS, TokenType.STAR, TokenType.SLASH]:
                    expr = AST.BinaryExpr(symbol=operator.value, left=left, right=right, type="double")
                else:
                    self.error("Invalid binary operator on doubles: ({}) {} ({})".format(left, operator, right))

            elif left.type in cpp_classes:
                if operator == TokenType.PLUS:
                    expr = self.create_function_call("add", [left, right])
                elif operator == TokenType.MINUS:
                    expr = self.create_function_call("subtract", [left, right])
                elif operator == TokenType.STAR and right.type == "double":
                    expr = self.create_function_call("scale", [left, right])
                elif operator == TokenType.STAR:
                    expr = self.create_function_call("multiply", [left, right])
                elif operator == TokenType.SLASH:
                    expr = self.create_function_call("divide", [left, right])
                else:
                    self.error("Invalid binary operator: ({}) {} ({})".format(left, operator, right))

            return self.parse_expression(precedence=precedence, left=expr)

        print(left, operator, right)
        self.error("Expect expression")

    def parse_arg_list(self, terminator: TokenType) -> list[AST.Expression]:
        args = []
        while not self.match(terminator):
            expr = self.parse_expression(1)
            if not isinstance(expr, AST.Expression):
                self.error("Function argument must be expression: " + str(expr))
            args.append(expr)
            self.match(TokenType.COMMA)
        return args

    def create_function_call(self, spec_name: str, args: list[AST.Expression]) -> AST.Expression:
        # Check as constructor
        as_class_name = spec_name.replace("_", " ").title().replace(" ", "")
        if as_class_name in cpp_classes:
            klass = cpp_classes[as_class_name]
            for func in klass.constructors:
                if func.match(args):
                    return AST.FunctionCall(func=func, args=args, type=func.return_type)

        # Check as method
        if len(args) > 0:
            as_class_name = args[0].type
            if as_class_name in cpp_classes:
                klass = cpp_classes[as_class_name]
                options = klass.get_methods(spec_name)
                for func in options:
                    if func.match(args[1:]):
                        return AST.FunctionCall(func=func, args=args, type=func.return_type)

        # Check as any static
        for klass in cpp_classes.values():
            for func in klass.get_methods(spec_name):
                if func.is_static and func.match(args):
                    return AST.FunctionCall(func=func, args=args, type=func.return_type)

        self.error("Undefined function: " + spec_name + " with args " + str([str(a) for a in args]))

    def create_field_access(self, spec_name: str, object: AST.Expression) -> AST.Expression:
        if object.type is None:
            self.error("object==" + str(object))
        object = object.dereference_all()
        if object.type not in cpp_classes:
            self.error("Unrecognised type in: " + str(object))

        klass = cpp_classes[object.type]

        # Check as field
        if spec_name in klass.get_fields():
            field: AST.FieldPrototype = klass.get_fields()[spec_name]
            return AST.FieldAccess(field=field, obj=object, type=field.type)

        # Check as getter
        for func in klass.get_methods(spec_name):
            if not func.is_static and func.match([]):
                return AST.FunctionCall(func=func, args=[object], type=func.return_type)

        self.error("Undefined field: " + spec_name + " on " + object.type)

    def get_var_type(self, name: str) -> Optional[str]:
        for i in range(len(self.scopes)):
            data = self.scopes[len(self.scopes) - i - 1]
            if name in data:
                return data[name]

        return None

    def put_var_type(self, name: str, type: str):
        self.scopes[len(self.scopes) - 1][name] = type

    def push_scope(self):
        self.scopes.append({})

    def pop_scope(self):
        del self.scopes[len(self.scopes) - 1]

    def get_err_context(self):
        if self.current_scenario is None:
            return super().get_err_context()
        else:
            return self.current_scenario


def find_feature_files(dirpath) -> list[str]:
    results = []

    for root, dirs, files in os.walk(dirpath):
        for name in files:
            if name.endswith(".feature"):
                results.append(os.path.join(root, name))

    return results


def parse_feature_files(feature_filepaths: list[str]) -> list[AST.Feature]:
    features = []

    for path in feature_filepaths:
        c = GherkinParser(path)
        features.append(c.build())

    return features

