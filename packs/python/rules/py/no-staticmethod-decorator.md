---
id: py/no-staticmethod-decorator
title: "@staticmethod, not parse = staticmethod(parse)"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLR0203]
since: 0.5.0
---

A method that the class body turns into a staticmethod by assignment, `parse = staticmethod(parse)`, is decorated `@staticmethod` instead: the fix writes the decorator above the `def` (above its other decorators) and removes the assignment, wherever in the class body it is. A method already decorated `@staticmethod` is not reported.

**Why:** The decorator says what the method is where it is defined; an assignment further down says it after the reader has read the method as an ordinary one.

```codelaw
match:
  kind: function_definition
  all:
    - has: {field: name, pattern: $N}
    - not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@staticmethod$'}}}
    - any:
        - all: [{inside: {kind: block, inside: {kind: class_definition}, has: {kind: expression_statement, pattern: $ASSIGN, has: {kind: assignment, pattern: $N = staticmethod($N)}}}}, {pattern: $DEF}]
        - all: [{inside: {kind: decorated_definition, pattern: $DEC, inside: {kind: block, inside: {kind: class_definition}, has: {kind: expression_statement, pattern: $ASSIGN, has: {kind: assignment, pattern: $N = staticmethod($N)}}}}}]
message: "Decorate `$N` with `@staticmethod`"
fix:
  - insert-before: { target: $DEF, text: "@staticmethod", line: true }
  - insert-before: { target: $DEC, text: "@staticmethod", line: true }
  - remove: $ASSIGN
```

## How to fix

1. Decorate the method `@staticmethod` and delete the assignment.

## Examples

### Python

#### Don't

```python
class Number:
    def parse(text):
        return int(text)
    parse = staticmethod(parse)
```

#### Do

```python
class Number:
    @staticmethod
    def parse(text):
        return int(text)
```
