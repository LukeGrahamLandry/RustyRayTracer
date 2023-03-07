from __future__ import annotations

import AST
from HeaderParser import *


class CodeGen:
    features: list[AST.Feature]
    filepath: str
    code: str
    output_line_count: int
    indentation_level: int
    total_scenario_count: int
    error_scenario_count: int
    includes: list[str]

    def __init__(self, features: list[AST.Feature], out_filepath: str, includes: list[str]):
        self.features = features
        self.filepath = out_filepath
        self.code = ""
        self.output_line_count = 0
        self.indentation_level = 0
        self.total_scenario_count = 0
        self.error_scenario_count = 0
        self.includes = includes

    def build(self):
        self.emit_header_boilerplate()
        for feature in self.features:
            self.emit_feature(feature)
        self.emit_footer_boilerplate()
        self.write()

    def emit_feature(self, feature: AST.Feature):
        self.push_scope()
        self.line("int _passedScenarioCount = 0;")
        self.line('cout << "FEATURE: {}" << endl;'.format(feature.name))
        for scenario in feature.scenarios:
            self.total_scenario_count += 1
            if isinstance(scenario, AST.ReportErr):
                self.error_scenario_count += 1
                self.line('cout << " - ERROR: {}" << endl;'.format(scenario.msg))
            else:
                self.emit_scenario(scenario)
        self.line("_totalPassedScenarioCount += _passedScenarioCount;")
        self.line('cout << "{} passed " << _passedScenarioCount << " of {} tests." << endl;'.format(feature.name, len(feature.scenarios)))
        self.pop_scope()

    def emit_scenario(self, scenario: AST.Scenario):
        starting_line_count = self.output_line_count
        self.push_scope()
        self.line("bool _scenarioPassed = true;")

        for stmt in scenario.background:
            self.emit_statement(stmt)
        for stmt in scenario.statements:
            self.emit_statement(stmt)

        self.line("if (_scenarioPassed){")
        self.line('    cout << " - PASS: {}" << endl;'.format(scenario.name))
        self.line("    _passedScenarioCount++;")
        self.line("} else {")
        self.line('    cout << " - FAIL: {}" << endl;'.format(scenario.name))
        self.line('    cout << "         at src/tests.cc:{}" << endl;'.format(starting_line_count))
        self.line("}")
        self.pop_scope()

    def emit_statement(self, stmt: AST.Statement):
        if isinstance(stmt, AST.Setter):
            # TODO
            pass
        elif isinstance(stmt, AST.Assertion):
            self.line("_scenarioPassed = _scenarioPassed && " + self.gen_expression(stmt.value) + ";")
        elif isinstance(stmt, AST.VarDeclare):
            if stmt.value is None:
                self.line("{} {};".format(stmt.type, stmt.variable.name))
            else:
                self.line("{} {} = {};".format(stmt.type, stmt.variable.name, self.gen_expression(stmt.value)))
        elif isinstance(stmt, AST.ExpressionStmt):
            self.line(self.gen_expression(stmt.value) + ";")
        else:
            raise ParseError("Not a statement: " + str(stmt))

    def gen_expression(self, expr: AST.Expression) -> str:
        """
        Recursively walk a single expression tree and return a string of source code.
        """

        if isinstance(expr, AST.VarAccess):
            return expr.name
        elif isinstance(expr, AST.FieldAccess):
            return self.gen_expression(expr.obj) + "." + expr.field.name
        elif isinstance(expr, AST.UnaryExpr):
            return "(" + expr.symbol + self.gen_expression(expr.value) + ")"
        elif isinstance(expr, AST.LiteralExpr):
            return expr.symbol
        elif isinstance(expr, AST.BinaryExpr):
            return "(" + self.gen_expression(expr.left) + " " + expr.symbol + " " + self.gen_expression(expr.right) + ")"
        elif isinstance(expr, AST.Dereference):
            return "(*" + self.gen_expression(expr.value) + ")"
        elif isinstance(expr, AST.AddressOf):
            return "(&" + self.gen_expression(expr.value) + ")"
        elif isinstance(expr, AST.FunctionCall):
            if expr.func.is_static:
                arg_str = "(" + ", ".join([self.gen_expression(e) for e in expr.args]) + ")"
                if expr.func.namespace is not None:
                    return expr.func.namespace + "::" + expr.func.name + arg_str
                else:  # no namespace means a global function
                    return expr.func.name + arg_str
            else:
                arg_str = "(" + ", ".join([self.gen_expression(e) for e in expr.args[1:]]) + ")"
                return self.gen_expression(expr.args[0]) + "." + expr.func.name + arg_str
        else:
            raise ParseError("Not an expression: " + str(expr))

    def emit_header_boilerplate(self):
        self.line("#include <chrono>")
        for file in self.includes:
            self.line('#include "' + file + '"')

        self.line("")
        self.line("// THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT MANUALLY.")
        self.line("int main()")
        self.push_scope()
        self.line("int _totalPassedScenarioCount = 0;")
        self.line("long _start_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();")

    def emit_footer_boilerplate(self):
        self.line("long _end_time = chrono::duration_cast< chrono::milliseconds >( chrono::system_clock::now().time_since_epoch()).count();")
        self.line('cout << "TOTAL: pass " << _totalPassedScenarioCount << ", fail " << ({0} - {1} - _totalPassedScenarioCount) << ", error {1}" << endl;'.format(self.total_scenario_count, self.error_scenario_count))
        self.line('cout << "' + ("=" * 30) + '" << endl;')
        self.line('cout << "- Execute: " << (_end_time - _start_time) << " ms." << endl;')
        self.line("return 0;")
        self.pop_scope()

    def push_scope(self):
        self.line("{")
        self.indentation_level += 1

    def pop_scope(self):
        self.indentation_level -= 1
        self.line("}")

    def line(self, c: str):
        self.code += ("    " * self.indentation_level) + c + "\n"
        self.output_line_count += 1

    def write(self):
        with open(self.filepath, "w") as f:
            f.write(self.code)
