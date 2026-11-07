---
id: py/type-name-incorrect-variance
title: "A TypeVar's name says its variance"
languages: [python]
fix: manual
fix_reason: "the new name is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLC0105, pylint/typevar-name-incorrect-variance]
since: 0.5.0
---

A covariant `TypeVar` is named with a `_co` suffix and a contravariant one with `_contra`: `T_co = TypeVar("T_co", covariant=True)`.

**Why:** The suffix is how a reader of a generic class's signature knows its variance.

```codelaw
match:
  kind: assignment
  all:
    - has: {field: right, kind: call, has: {field: function, regex: '^(typing\.|typing_extensions\.)?TypeVar$'}}
    - any:
        - all:
            - has: {field: right, has: {field: arguments, has: {kind: keyword_argument, regex: '^covariant\s*=\s*True$'}}}
            - has: {field: left, kind: identifier, not: {regex: '_co$'}}
        - all:
            - has: {field: right, has: {field: arguments, has: {kind: keyword_argument, regex: '^contravariant\s*=\s*True$'}}}
            - has: {field: left, kind: identifier, not: {regex: '_contra$'}}
message: "Name the TypeVar with its variance's suffix"
```

## How to fix

1. Rename the type variable with `_co` (covariant) or `_contra` (contravariant), in the call's string too, and update its uses.

## Examples

### Python

#### Don't

```python
T = TypeVar("T", covariant=True)
```

#### Do

```python
T_co = TypeVar("T_co", covariant=True)
```
