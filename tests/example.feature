Feature: Who watches the watcher

Scenario: Addition
  Given a ← 1
    And b ← 2
   Then a + b = 3

Scenario: Division
  Given a ← 4
    And b ← 2
   Then a / b = 2
    And b / a = 0.5

Scenario: Root
  Given a ← 4
   Then √a = 2

Scenario: Root precedence and double unary op
  Given a ← -√2/2
   Then a = -0.7071067
