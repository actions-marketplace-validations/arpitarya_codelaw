---
id: py/type-bivariance
title: "A TypeVar is not both covariant and contravariant"
languages: [python]
fix: manual
fix_reason: "which variance was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLC0131, pylint/typevar-double-variance]
since: 0.5.0
---

A `TypeVar` is not declared with both `covariant=True` and `contravariant=True`.

**Why:** No type can be both; the declaration is an error type checkers report.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^(typing\.|typing_extensions\.)?TypeVar$'}
    - has: {field: arguments, has: {kind: keyword_argument, regex: '^covariant\s*=\s*True$'}}
    - has: {field: arguments, has: {kind: keyword_argument, regex: '^contravariant\s*=\s*True$'}}
message: "A TypeVar is covariant, contravariant or neither"
```

## How to fix

1. Keep the variance the type parameter has, and drop the other keyword.

## Examples

### Python

#### Don't

```python
T = TypeVar("T", covariant=True, contravariant=True)
```

#### Do

```python
T_co = TypeVar("T_co", covariant=True)
```
