---
id: py/invalid-class-object
title: "__class__ is assigned a class"
languages: [python]
fix: manual
fix_reason: "which class was meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-class-object]
since: 0.5.0
---

`obj.__class__` is not assigned a literal (a number, a string, `None`, a list …).

**Why:** Python raises `TypeError`: `__class__` must be a class.

```codelaw
match:
  kind: assignment
  all:
    - has: {field: left, kind: attribute, regex: '\.__class__$'}
    - has: {field: right, any: [{kind: integer}, {kind: float}, {kind: string}, {kind: none}, {kind: 'true'}, {kind: 'false'}, {kind: list}, {kind: dictionary}, {kind: tuple}]}
message: "`__class__` must be a class"
```

## How to fix

1. Assign the class object itself.

## Examples

### Python

#### Don't

```python
obj.__class__ = "Other"
```

#### Do

```python
obj.__class__ = Other
```
