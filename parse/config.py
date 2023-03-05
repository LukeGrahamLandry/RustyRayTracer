from common import *
from HeaderParser import *

includes = ["common.h", "Matrix.h", "Tuple.h", "Colour.h", "Canvas.h", "Ray.h", "World.h", "Camera.h", "shapes/Plane.h"]

binary_operators = [
    OperatorDef(TokenType.STAR, "Matrix", "Matrix", "Matrix", "<a>.multiply(<b>)"),
    OperatorDef(TokenType.STAR, "Matrix", "Tuple", "Tuple", "<a>.multiply(<b>)"),
    OperatorDef(TokenType.PLUS, "Tuple", "Tuple", "Tuple", "<a>.add(<b>)"),
    OperatorDef(TokenType.MINUS, "Tuple", "Tuple", "Tuple", "<a>.subtract(<b>)"),
    OperatorDef(TokenType.STAR, "Tuple", "double", "Tuple", "<a>.scale(<b>)"),
    OperatorDef(TokenType.SLASH, "Tuple", "double", "Tuple", "<a>.divide(<b>)"),
    OperatorDef(TokenType.PLUS, "Colour", "Colour", "Colour", "<a>.add(<b>)"),
    OperatorDef(TokenType.MINUS, "Colour", "Colour", "Colour", "<a>.subtract(<b>)"),
    OperatorDef(TokenType.STAR, "Colour", "double", "Colour", "<a>.scale(<b>)"),
    OperatorDef(TokenType.STAR, "Colour", "Colour", "Colour", "<a>.multiply(<b>)")
]

for op in [TokenType.STAR, TokenType.PLUS, TokenType.SLASH, TokenType.MINUS]:
    binary_operators.append(OperatorDef(op, "double", "double", "double", "<a> " + op.value + " <b>"))

unary_operators = [
    OperatorDef(TokenType.ROOT, "", "double", "double", "sqrt(<b>)"),
    OperatorDef(TokenType.MINUS, "", "double", "double", "-<b>"),
    OperatorDef(TokenType.MINUS, "", "Tuple", "Tuple", "<b>.negate()"),
    OperatorDef(TokenType.BANG, "", "bool", "bool", "!<b>")
]

# Could just keep line breaks as a token type, but I like the idea of insignificant whitespace.
terminators = [TokenType.EOF, TokenType.AND, TokenType.GIVEN, TokenType.THEN, TokenType.WHEN, TokenType.SCENARIO,
               TokenType.COMMA, TokenType.RIGHT_PAREN, TokenType.RIGHT_BRACKET, TokenType.SCENARIO_OUTLINE]

variables = {
    "identity_matrix": Expr(c_code="Transformation::identity()", type="Matrix")
}

getter_collections = {
    "Intersections": "Intersection"
}

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
