---
id: py/invalid-slots-object
title: "__slots__ lists only strings"
languages: [python]
fix: manual
fix_reason: "the attribute's name is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-slots-object]
since: 0.5.0
---

Each item of a `__slots__` list, tuple or set is a non-empty string.

**Why:** Python raises `TypeError` at class creation for anything else.

```codelaw
match:
  any: [{kind: integer}, {kind: float}, {kind: none}, {kind: 'true'}, {kind: 'false'}, {kind: string, regex: '^[rbuRBU]*(""|'''')$'}]
  inside:
    any: [{kind: list}, {kind: tuple}, {kind: set}]
    inside: {kind: assignment, field: right, has: {field: left, regex: '^__slots__$'}}
message: "A `__slots__` item must be an attribute name"
```

## How to fix

1. Replace the item with the attribute's name as a string.

## Examples

### Python

#### Don't

```python
class A:
    __slots__ = ("a", 1)
```

#### Do

```python
class A:
    __slots__ = ("a", "b")
```
