from __future__ import annotations

from enum import Enum


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
    BANG = "!"
    MINUS = "-"
    SLASH = "/"
    LEFT_PAREN = "("
    RIGHT_PAREN = ")"
    LEFT_BRACKET = "["
    RIGHT_BRACKET = "]"
    LEFT_BRACE = "{"
    RIGHT_BRACE = "}"
    PI = "π"
    COMMA = ","
    ROOT = "√"
    PIPE = "|"
    STRING = 3
    EOF = 2
    DOT = 4
    SCENARIO_OUTLINE = "Scenario Outline: "
    AMP = "&"
    CONST = "const"
    VIRTUAL = "virtual"
    OVERRIDE = "override"
    PUBLIC = "public"
    PRIVATE = "private"
    CLASS = "class"
    STATIC = "static"
    COLON = ":"
    SEMICOLON = ";"
    INLINE = "inline"


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


def scan(src: str, keywords=dict[str, TokenType]) -> list[Token]:
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

        if src[start] == "#" or (src[start] == "/" and (start + 1) < len(src) and src[start + 1] == "/"):
            while start < len(src) and src[start] != "\n":
                start += 1

            continue

        lexeme = ""
        while start < len(src) and src[start] != " " and src[start] != "\n" and src[start] != "#" and not (src[start] == "/" and (start + 1) < len(src) and src[start + 1] == "/"):
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


class AbstractParser:
    tokens: list[Token]
    i: int
    filepath: str

    def __init__(self, path: str, keywords):
        self.filepath = path
        with open(path, "r") as f:
            src = f.read()

        self.tokens = scan(src, keywords)
        self.i = 0

    def read_name(self) -> str:
        return self.consume(TokenType.STRING, "Expect string.").lexeme

    def identifier(self) -> str:
        return self.consume(TokenType.IDENTIFIER, "Expect identifier.").lexeme

    def match(self, type: TokenType) -> bool:
        if self.check(type):
            self.i += 1
            return True
        return False

    def check(self, type: TokenType) -> bool:
        return self.peek().type == type

    def isDone(self) -> bool:
        return self.match(TokenType.EOF)

    def peek(self) -> Token:
        if self.i >= len(self.tokens):
            self.i = len(self.tokens) - 1
            return self.tokens[-1]  # will be EOF

        return self.tokens[self.i]

    def advance(self) -> Token:
        self.i += 1
        return self.tokens[self.i - 1]

    def consume(self, type: TokenType, err: str) -> Token:
        if not self.match(type):
            self.i += 1
            self.error(err)

        return self.tokens[self.i - 1]

    def advance_until(self, type: TokenType) -> bool:
        while True:
            if self.match(TokenType.EOF):
                return False

            if self.match(type):
                return True

            self.advance()

    def get_current_line_token_string(self) -> str:
        line_num = self.tokens[self.i - 1].line
        # Inefficient but only on error so who cares
        this_line = [(i, str(t)) for i, t in enumerate(self.tokens) if t.line == line_num]
        s = "    - "
        for i, token in this_line:
            if i == self.i - 1:
                s += "[" + str(token) + "], "
            else:
                s += "(" + str(token) + "), "
        return s[:-2]

    def error(self, err):
        line_num = self.tokens[self.i - 1].line
        print("Error on line {} ({}).\n    - {}".format(line_num, self.get_err_context(), err))
        print(self.get_current_line_token_string())
        raise ParseError()

    def get_err_context(self):
        return self.filepath
