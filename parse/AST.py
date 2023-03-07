from dataclasses import dataclass, field
from typing import Optional


@dataclass
class FunctionPrototype:
    name: str
    is_static: bool
    return_type: str
    argument_types: list[str] = field(default_factory=lambda: [])
    namespace: str | None = None

    def match(self, args: list["Expression"]) -> bool:
        if len(self.argument_types) != len(args):
            return False

        for i, type in enumerate(self.argument_types):
            if type != args[i].type:
                return False

        return True


FunctionPrototype.SQRT = FunctionPrototype(name="sqrt", is_static=True, return_type="double", argument_types=["double"])


@dataclass
class FieldPrototype:
    name: str
    type: str
    is_static: bool
    namespace: str | None = None


@dataclass
class ClassPrototype:
    name: str
    filename: str
    is_abstract: bool = False
    fields: list[FieldPrototype] = field(default_factory=lambda: [])
    methods: list[FunctionPrototype] = field(default_factory=lambda: [])
    constructors: list[FunctionPrototype] = field(default_factory=lambda: [])
    extends: str | None = None

    def __str__(self):
        if self.is_abstract:
            s = "Abstract Class: "
        else:
            s = "Class: "

        s += self.name
        if self.extends is not None:
            s += " extends " + self.extends
        s += "\n  - Location: " + self.filename
        s += "\n  - Fields:"
        for f in self.fields:
            s += "\n    - " + str(f)
        s += "\n  - Constructors:"
        for f in self.constructors:
            s += "\n    - " + str(f)
        s += "\n  - Methods:"
        for f in self.methods:
            s += "\n    - " + str(f)
        return s

    def get_fields(self):
        return {obj.name: obj for obj in self.fields}

    # multiple options for one name because c++ allows overloading with different type signatures
    def get_methods(self, name: str) -> list[FunctionPrototype]:
        return [obj for obj in self.methods if obj.name == name]


@dataclass
class Expression:
    type: str

    def dereference(self) -> "Expression":
        return Dereference(self.type[:-1], self)

    def dereference_all(self) -> "Expression":
        expr = self
        for i in range(self.count_pointer_indirection()):
            expr = expr.dereference()
        return expr

    def address_of(self) -> "Expression":
        return AddressOf(self.type + "*", self)

    def count_pointer_indirection(self) -> int:
        if self.type is None:
            return 0

        count = 0
        s = self.type
        while s.endswith("*"):
            count += 1
            s = s[:-1]

        return count

    def match_pointer_indirection(self, other: "Expression") -> "Expression":
        expr = self

        while expr.count_pointer_indirection() > other.count_pointer_indirection():
            expr = expr.dereference()

        while expr.count_pointer_indirection() < other.count_pointer_indirection():
            expr = expr.address_of()

        return expr


@dataclass
class Dereference(Expression):
    value: Expression


@dataclass
class AddressOf(Expression):
    value: Expression


@dataclass
class FunctionCall(Expression):
    func: FunctionPrototype
    args: list[Expression]


@dataclass
class BinaryExpr(Expression):
    symbol: str
    left: Expression
    right: Expression


@dataclass
class UnaryExpr(Expression):
    symbol: str
    value: Expression


@dataclass
class LiteralExpr(Expression):
    symbol: str


@dataclass
class FieldAccess(Expression):
    field: FieldPrototype
    obj: Expression


@dataclass
class VarAccess(Expression):
    name: str


@dataclass
class Setter:
    variable: FieldAccess | VarAccess
    value: Expression


@dataclass
class VarDeclare:
    variable: FieldAccess | VarAccess
    value: Optional[Expression]
    type: str


@dataclass
class Assertion:
    value: Expression


@dataclass
class Scenario:
    name: str
    statements: list["Statement"]
    background: list["Statement"]

@dataclass
class ReportErr:
    msg: str


@dataclass
class Feature:
    name: str
    scenarios: list[Scenario | ReportErr]


@dataclass
class ExpressionStmt:
    value: Expression


Statement = Setter | Assertion | VarDeclare | ExpressionStmt
