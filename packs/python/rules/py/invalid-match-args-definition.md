---
id: py/invalid-match-args-definition
title: "__match_args__ is a tuple of strings"
languages: [python]
fix: manual
fix_reason: "the attribute names are the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-match-args-definition]
since: 0.5.0
---

`__match_args__` is assigned a tuple (of attribute names), not a list or another value.

**Why:** Python raises `TypeError` at the first `case` that uses positional patterns.

```codelaw
match:
  kind: assignment
  inside: {kind: expression_statement, inside: {kind: block, inside: {kind: class_definition}}}
  all:
    - has: {field: left, regex: '^__match_args__$'}
    - has: {field: right, not: {kind: tuple}}
message: "`__match_args__` must be a tuple of strings"
```

## How to fix

1. Write it as a tuple of the attribute names: `("x", "y")`.

## Examples

### Python

#### Don't

```python
class P:
    __match_args__ = ["x", "y"]
```

#### Do

```python
class P:
    __match_args__ = ("x", "y")
```
