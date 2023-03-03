from common import *
from header import *

keywords = {e.value: e for e in TokenType if isinstance(e.value, str)}
functions = {
    "color": FunctionDef(False, "Colour", "Colour"),
    "plane": FunctionDef(False, "Plane", "Plane"),
    "point_light": FunctionDef(False, "PointLight", "PointLight"),
    "material": FunctionDef(False, "Material", "Material"),
    "camera": FunctionDef(False, "Camera", "Camera"),
    "ray": FunctionDef(False, "Ray", "Ray"),
    "sphere": FunctionDef(False, "Sphere", "Sphere"),
    "intersection": FunctionDef(False, "Intersection", "Intersection"),
    "point": FunctionDef(False, "Tuple", "Point"),
    "vector": FunctionDef(False, "Tuple", "Vector"),
    "tuple": FunctionDef(False, "Tuple", "Tuple"),
    "inverse": FunctionDef(True, "Matrix", "inverse"),
    "magnitude": FunctionDef(True, "double", "magnitude"),
    "normalize": FunctionDef(True, "Tuple", "normalize"),
    "dot": FunctionDef(True, "double", "dot"),
    "cross": FunctionDef(True, "Tuple", "cross"),
    "position": FunctionDef(True, "Tuple", "position"),
    "intersect": FunctionDef(True, "Intersections", "intersect"),
    "hit": FunctionDef(True, "Intersection", "hit"),
    "transform": FunctionDef(True, "Ray", "transform"),
    "set_transform": FunctionDef(True, "void", "set_transform"),
    "normal_at": FunctionDef(True, "Tuple", "normal_at"),
    "reflect": FunctionDef(True, "Tuple", "reflect"),
    "lighting": FunctionDef(True, "Colour", "lighting"),
    "intersect_world": FunctionDef(True, "Intersections", "intersect"),
    "default_world": FunctionDef(False, "World", "World::default_world"),
    "prepare_computations": FunctionDef(True, "IntersectionComps", "prepare_computations"),
    "shade_hit": FunctionDef(True, "Colour", "shade_hit"),
    "getShape": FunctionDef(True, "Shape*", "getShape"),
    "color_at": FunctionDef(True, "Colour", "color_at"),
    "ray_for_pixel": FunctionDef(True, "Ray", "ray_for_pixel"),
    "render": FunctionDef(True, "Canvas", "render"),
    "pixel_at": FunctionDef(True, "Colour", "pixel_at"),
    "is_shadowed": FunctionDef(True, "bool", "is_shadowed"),
    "getLight": FunctionDef(True, "PointLight*", "getLight"),
    "isPoint": FunctionDef(True, "bool", "isPoint"),
    "isVector": FunctionDef(True, "bool", "isVector"),
    "local_normal_at": FunctionDef(True, "Tuple", "local_normal_at"),
    "local_intersect": FunctionDef(True, "Intersections", "local_intersect")
}

transformations = ["translation", "scaling", "rotation_x", "rotation_y", "rotation_z", "shearing", "view_transform"]
for name in transformations:
    functions[name] = FunctionDef(False, "Matrix", "Transformation::" + name)

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

Field = namedtuple("Field", "name type is_getter")

fields = {
    "Colour": [
        Field(name="red", type="double", is_getter=False),
        Field(name="green", type="double", is_getter=False),
        Field(name="blue", type="double", is_getter=False)
    ],
    "Ray": [
        Field(name="origin", type="Tuple", is_getter=False),
        Field(name="direction", type="Tuple", is_getter=False)
    ],
    "Intersection": [
        Field(name="t", type="double", is_getter=False),
        Field(name="object", type="Shape*", is_getter=False)
    ],
    "IntersectionComps": [
        Field(name="t", type="double", is_getter=False),
        Field(name="object", type="Shape*", is_getter=False),
        Field(name="point", type="Tuple", is_getter=False),
        Field(name="normalv", type="Tuple", is_getter=False),
        Field(name="eyev", type="Tuple", is_getter=False),
        Field(name="inside", type="bool", is_getter=False)
    ],
    "Tuple": [
        Field(name="x", type="double", is_getter=True),
        Field(name="y", type="double", is_getter=True),
        Field(name="z", type="double", is_getter=True),
        Field(name="w", type="double", is_getter=True)
    ],
    "Intersections": [
        Field(name="count", type="double", is_getter=True)
    ],
    "Shape": [
        Field(name="transform", type="Matrix", is_getter=False),
        Field(name="material", type="Material", is_getter=False)
    ],
    "Sphere": [
        Field(name="transform", type="Matrix", is_getter=False),
        Field(name="material", type="Material", is_getter=False)
    ],
    "Plane": [
        Field(name="transform", type="Matrix", is_getter=False),
        Field(name="material", type="Material", is_getter=False)
    ],
    "PointLight": [
        Field(name="intensity", type="Colour", is_getter=False),
        Field(name="position", type="Tuple", is_getter=False)
    ],
    "Material": [
        Field(name="color", type="Colour", is_getter=False),
        Field(name="ambient", type="double", is_getter=False),
        Field(name="diffuse", type="double", is_getter=False),
        Field(name="specular", type="double", is_getter=False),
        Field(name="shininess", type="double", is_getter=False)
    ],
    "Camera": [
        Field(name="hsize", type="double", is_getter=False),
        Field(name="vsize", type="double", is_getter=False),
        Field(name="field_of_view", type="double", is_getter=False),
        Field(name="transform", type="Matrix", is_getter=False),
        Field(name="pixel_size", type="double", is_getter=False)
    ]
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
