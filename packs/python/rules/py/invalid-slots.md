---
id: py/invalid-slots
title: "__slots__ is a sequence of names"
languages: [python]
fix: manual
fix_reason: "which attributes the class has is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-slots]
since: 0.5.0
---

`__slots__` is assigned a string, or a list, tuple or set of strings — not a number, `None` or a boolean.

**Why:** Python raises `TypeError` when the class is created.

```codelaw
match:
  kind: assignment
  inside: {kind: expression_statement, inside: {kind: block, inside: {kind: class_definition}}}
  all:
    - has: {field: left, regex: '^__slots__$'}
    - has: {field: right, any: [{kind: integer}, {kind: float}, {kind: none}, {kind: 'true'}, {kind: 'false'}]}
message: "`__slots__` must name the attributes"
```

## How to fix

1. List the attribute names: `__slots__ = ("a", "b")`.

## Examples

### Python

#### Don't

```python
class A:
    __slots__ = 1
```

#### Do

```python
class A:
    __slots__ = ("n",)
```
