---
id: py/no-classmethod-decorator
title: "@classmethod, not from_text = classmethod(from_text)"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLR0202]
since: 0.5.0
---

A method that the class body turns into a classmethod by assignment, `from_text = classmethod(from_text)`, is decorated `@classmethod` instead: the fix writes the decorator above the `def` (above its other decorators) and removes the assignment, wherever in the class body it is. A method already decorated `@classmethod` is not reported.

**Why:** The decorator says what the method is where it is defined; an assignment further down says it after the reader has read the method as an ordinary one.

```codelaw
match:
  kind: function_definition
  all:
    - has: {field: name, pattern: $N}
    - not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@classmethod$'}}}
    - any:
        - all: [{inside: {kind: block, inside: {kind: class_definition}, has: {kind: expression_statement, pattern: $ASSIGN, has: {kind: assignment, pattern: $N = classmethod($N)}}}}, {pattern: $DEF}]
        - all: [{inside: {kind: decorated_definition, pattern: $DEC, inside: {kind: block, inside: {kind: class_definition}, has: {kind: expression_statement, pattern: $ASSIGN, has: {kind: assignment, pattern: $N = classmethod($N)}}}}}]
message: "Decorate `$N` with `@classmethod`"
fix:
  - insert-before: { target: $DEF, text: "@classmethod", line: true }
  - insert-before: { target: $DEC, text: "@classmethod", line: true }
  - remove: $ASSIGN
```

## How to fix

1. Decorate the method `@classmethod` and delete the assignment.

## Examples

### Python

#### Don't

```python
class Number:
    def from_text(cls, text):
        return cls(text)
    from_text = classmethod(from_text)
```

#### Do

```python
class Number:
    @classmethod
    def from_text(cls, text):
        return cls(text)
```
