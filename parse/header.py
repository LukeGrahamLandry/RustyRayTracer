from __future__ import annotations
from common import *

header_keywords = dict({e.value: e for e in [
    TokenType.AMP,
    TokenType.CONST,
    TokenType.VIRTUAL,
    TokenType.OVERRIDE,
    TokenType.EQUALITY,
    TokenType.ASSIGN,
    TokenType.STAR,
    TokenType.LEFT_PAREN,
    TokenType.RIGHT_PAREN,
    TokenType.LEFT_BRACE,
    TokenType.RIGHT_BRACE,
    TokenType.COMMA,
    TokenType.PUBLIC,
    TokenType.PRIVATE,
    TokenType.CLASS,
    TokenType.STATIC,
    TokenType.COLON,
    TokenType.SEMICOLON,
    TokenType.INLINE
]})


class HeaderParser(AbstractParser):
    classes: list[ClassPrototype]
    current_class: ClassPrototype | None

    def __init__(self, filepath: str):
        super().__init__(filepath, header_keywords)
        self.classes = []
        self.current_class = None

    def parse(self) -> list[ClassPrototype]:
        while not self.match(TokenType.EOF):
            try:
                if self.start_class():
                    self.parse_class_body()
            except ParseError:
                pass

        return self.classes

    def start_class(self) -> bool:
        while True:
            if self.match(TokenType.CLASS):
                name = self.identifier()
                if self.match(TokenType.SEMICOLON):
                    # forward definition
                    continue
                else:
                    break

            if self.match(TokenType.EOF):
                return False

            self.advance()

        extends = None
        if self.match(TokenType.COLON):
            self.match(TokenType.PRIVATE)
            self.match(TokenType.PUBLIC)
            extends = self.identifier()

        self.consume(TokenType.LEFT_BRACE, "Expect '{' before class body.")

        self.current_class = ClassPrototype(name=name, filename=self.filepath, extends=extends)
        self.classes.append(self.current_class)

        return True

    def parse_class_body(self):
        while not self.match(TokenType.EOF) and not self.match(TokenType.RIGHT_BRACE):
            self.parse_property_definition()

        self.consume(TokenType.SEMICOLON, "Expect ';' after class body.")
        self.current_class = None

    def parse_property_definition(self):
        if self.match(TokenType.PUBLIC):
            self.consume(TokenType.COLON, "Expect ':' after 'public'.")
        if self.match(TokenType.PRIVATE):
            self.consume(TokenType.COLON, "Expect ':' after 'private'.")

        self.match(TokenType.INLINE)
        is_static = self.match(TokenType.STATIC)
        is_virtual = self.match(TokenType.VIRTUAL)

        return_type = self.parse_type()
        is_static = is_static or self.match(TokenType.STATIC)
        if return_type.startswith("~"):   # destructor
            # don't care. just consume the body
            self.match(TokenType.LEFT_PAREN)
            self.parse_arg_list()

        elif self.match(TokenType.LEFT_PAREN):  # constructor
            func = FunctionPrototype(name=self.current_class.name, return_type=self.current_class.name, is_static=True)
            func.argument_types = self.parse_arg_list()
            self.current_class.constructors.append(func)

        else:
            name = self.identifier()
            if self.match(TokenType.LEFT_PAREN):   # function
                func = FunctionPrototype(name=name, return_type=return_type, is_static=is_static)
                func.argument_types = self.parse_arg_list()
                self.current_class.methods.append(func)
            else:  # field
                field = FieldPrototype(name=name, type=return_type, is_static=is_static)
                self.current_class.fields.append(field)
                self.consume(TokenType.SEMICOLON, "Expect ';' after field definition.")

    def parse_arg_list(self) -> list[str]:
        args = []

        if not self.match(TokenType.RIGHT_PAREN):
            while not self.isDone():
                args.append(self.parse_type())
                self.match(TokenType.IDENTIFIER)

                if self.match(TokenType.EQUALITY):  # default parameter
                    while self.peek().type not in [TokenType.EOF, TokenType.RIGHT_PAREN, TokenType.COMMA]:
                        self.advance()

                if self.match(TokenType.RIGHT_PAREN):
                    break

                self.consume(TokenType.COMMA, "Expect ',' between parameters.")

        self.match(TokenType.CONST)
        self.match(TokenType.OVERRIDE)

        if self.match(TokenType.COLON):
            while self.peek().type not in [TokenType.EOF, TokenType.LEFT_BRACE, TokenType.EQUALITY, TokenType.SEMICOLON]:
                if self.match(TokenType.IDENTIFIER) and self.match(TokenType.LEFT_BRACE) and self.match(TokenType.IDENTIFIER) and self.match(TokenType.RIGHT_BRACE):
                    # initializer list
                    continue

                self.advance()

        if self.match(TokenType.LEFT_BRACE):
            depth = 1
            while depth > 0:
                if self.match(TokenType.EOF):
                    self.error("Expect '}' after inline function body.")

                if self.check(TokenType.RIGHT_BRACE):
                    depth -= 1

                if self.check(TokenType.LEFT_BRACE):
                    depth += 1

                self.advance()

            self.match(TokenType.SEMICOLON)

        else:
            if self.match(TokenType.EQUALITY):
                self.match(TokenType.IDENTIFIER)  # default, delete
                if self.match(TokenType.NUMBER):
                    self.current_class.is_abstract = True

            self.consume(TokenType.SEMICOLON, "Expect ';' after function definition.")

        return args

    def parse_type(self) -> str:
        self.match(TokenType.CONST)

        type = self.identifier()
        while self.match(TokenType.STAR):
            type += "*"

        self.match(TokenType.AMP)
        return type

    def get_err_context(self):
        if self.current_class is None:
            return super().get_err_context()
        else:
            return self.current_class.name


def walk(dirpath) -> list[ClassPrototype]:
    classes = []
    for root, dirs, files in os.walk(dirpath):
        for name in files:
            if not name.endswith(".h"):
                continue

            path = os.path.join(root, name)
            classes += HeaderParser(path).parse()

    return classes


if __name__ == "__main__":
    [print(str(s)) for s in walk("../src")]
