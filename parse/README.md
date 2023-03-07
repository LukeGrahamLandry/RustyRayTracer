- These scripts expect to be run from the project root directory (ie `.` not .`/parse`), to make the relative file paths work out. 
- tests.py has cmake binary paths hardcoded for MacOS

## AbstractParser

Boilerplate scanner and parser implementation. 

- scan: walk through a string and generate a list of Tokens
- parse: walk through a list of Tokens

## AST

Data classes for representing abstract syntax tree nodes and class prototypes.

## HeaderParser

Reads c++ header files and generates a list of ClassPrototypes. 

TODO: support generics, default arguments, and global functions. 

## GherkinParser

Reads .feature files (like the ones used in The Ray Tracer Challenge) and generates an abstract syntax tree. 

TODO: support matrix literals, and tables of field setters.

## CodeGen

Walks an abstract syntax tree and generate c++ source code. 

## tests

- Run the header parser on tests/example.h and compare it to the expected ClassPrototypes. 
- Run the gherkin parser and code gen on tests/example.feature and run the generated code. 
- Run the tests from The Raytracer Challenge (tests/book/*.feature).
    - https://media.pragprog.com/titles/jbtracer/code/jbtracer-code.zip
