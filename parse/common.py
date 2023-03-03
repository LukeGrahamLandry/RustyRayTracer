from __future__ import annotations
from dataclasses import dataclass, field
from base_parser import *

import os
from collections import namedtuple
from enum import Enum
from time import time


class Expr:
    c_code: str
    type: str

    def __init__(self, c_code: str, type: str):
        self.c_code = c_code
        self.type = type

    def dereference(self):
        while self.type.endswith("*"):
            self.type = self.type[:-1]
            self.c_code = "(*" + self.c_code + ")"

    def address_of(self):
        self.type = self.type + "*"
        self.c_code = "(&" + self.c_code + ")"


class FunctionDef:
    is_method: bool
    return_type: str
    c_name: str | None
    call_format: str

    def __init__(self, is_method: bool, return_type: str, c_name: str | None):
        self.is_method = is_method
        self.return_type = return_type
        self.c_name = c_name
        if self.is_method:
            self.call_format = "{0}.{1}({2})"
        else:
            self.call_format = "{1}({0}, {2})"

    def get_call_expr(self, args: list[Expr]) -> Expr:
        # cringe. I should know arity ahead of time
        if len(args) == 0:
            first_arg = ""
            others = ""
        elif len(args) == 1:
            first_arg = args[0].c_code
            others = ""
        else:
            # cringe
            if self.c_name == "Intersection" and "*" in args[1].type:
                args[1] = Expr(c_code="*" + args[1].c_code, type="Shape")

            first_arg = args[0].c_code
            others = ", ".join([arg.c_code for arg in args[1:]])

        c_code = self.call_format.format(first_arg, self.c_name, others)
        c_code = c_code.replace(", )", ")")
        return Expr(c_code=c_code, type=self.return_type)


class Field:
    name: str
    type: Klass
    template: str

    def __init__(self, name: str, type: Klass, is_getter=False):
        self.name = name
        self.type = type

        self.template = "{0}." + self.name
        if is_getter:
            self.template += "()"

    def get_expr(self, owner: Expr) -> Expr:
        return Expr(c_code=self.template.format(owner.c_code), type=self.type)


class Klass:
    name: str
    methods: list[FunctionDef]
    fields: list[Field]
    extends: Klass | None
    is_abstract: bool

    def __init__(self, name: str, methods: list[FunctionDef] | None = None, fields: list[Field] | None = None,
                 extends: Klass | None = None, is_abstract=False):
        self.name = name
        if methods is None:
            self.methods = []
        else:
            self.methods = methods

        if fields is None:
            self.fields = []
        else:
            self.fields = fields

        self.extends = extends
        self.is_abstract = True


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


@dataclass
class FunctionPrototype:
    name: str
    is_static: bool
    return_type: str
    argument_types: list[str] = field(default_factory=lambda: [])


@dataclass
class FieldPrototype:
    name: str
    type: str
    is_static: bool


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

        s +=  self.name
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
