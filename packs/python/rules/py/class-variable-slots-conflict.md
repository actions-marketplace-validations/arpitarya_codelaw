---
id: py/class-variable-slots-conflict
title: "A __slots__ name is not also a class variable"
languages: [python]
fix: manual
fix_reason: "the default moves into `__init__`, or the name out of `__slots__`"
fix_exception: restructure
stability: preview
ported_from: [pylint/class-variable-slots-conflict]
since: 0.5.0
---

A name listed in a class's `__slots__` is not also assigned a value in the class body (an annotation alone is fine).

**Why:** Python raises `ValueError` when the class is created: the class variable and the slot descriptor would share the name.

```codelaw
match:
  kind: string_content
  pattern: $N
  inside:
    kind: assignment
    stopBy: end
    has: {field: left, regex: '^__slots__$'}
    inside:
      kind: expression_statement
      inside:
        kind: block
        has: {kind: expression_statement, has: {kind: assignment, all: [{has: {field: left, kind: identifier, pattern: $N}}, {has: {field: right, regex: '.'}}]}}
message: "`$N` is both a slot and a class variable"
```

## How to fix

1. Set the default in `__init__`, or drop the name from `__slots__`.

## Examples

### Python

#### Don't

```python
class A:
    __slots__ = ("x",)
    x = 1
```

#### Do

```python
class A:
    __slots__ = ("x",)

    def __init__(self):
        self.x = 1
```
