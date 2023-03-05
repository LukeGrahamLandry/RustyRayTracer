from dataclasses import dataclass
from typing import Optional

from common import *


@dataclass
class Expression:
    type: str


@dataclass
class FunctionCall(Expression):
    func: FunctionPrototype
    args: list["Expression"]


@dataclass
class FieldAccess(Expression):
    field: FieldPrototype
    obj: "Expression"


@dataclass
class VarAccess(Expression):
    name: str


@dataclass
class Setter:
    variable: FieldAccess | VarAccess
    value: "Expression"


@dataclass
class VarDeclare:
    variable: FieldAccess | VarAccess
    value: Optional["Expression"]
    type: str


@dataclass
class Assertion:
    value: "Expression"


@dataclass
class Scenario:
    name: str
    statements: list["Statement"]


@dataclass
class ReportErr:
    msg: str


@dataclass
class Feature:
    name: str
    scenarios: list[Scenario | ReportErr]


@dataclass
class ExpressionStmt:
    value: "Expression"


Statement = Setter | Assertion | VarDeclare | ExpressionStmt
