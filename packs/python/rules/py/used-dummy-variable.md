---
id: py/used-dummy-variable
title: "A local variable named like a dummy is not read"
languages: [python]
fix: suggest
fix_reason: "the rename is certain only when the plain name (or the name with a trailing underscore) is free in every scope that sees the variable; where it is not, no edit is offered"
stability: preview
ported_from: [ruff/RUF052]
since: 0.5.0
---

A local variable of a function (or a comprehension) whose name starts with an underscore (`_x`, not `_`, `__`, or a dunder) is assigned and then read. A leading underscore says a name is unused. Parameters are not reported, nor are names declared `global` or `nonlocal`, nor `T = TypeVar("T")` and its kind. The fix renames the variable to the name without its underscores, or with one trailing underscore when that shadows a builtin or a name in scope.

**Why:** Readers and linters treat `_name` as a throwaway, so a read of it is a surprise.

```codelaw
native: py/used-dummy-variable
```

## How to fix

1. Rename the variable without the leading underscore.

## Examples

### Python

#### Don't

```python
def total(items):
    _sum = 0
    for item in items:
        _sum += item
    return _sum
```

#### Fixed

```python
def total(items):
    sum_ = 0
    for item in items:
        sum_ += item
    return sum_
```

#### Do

```python
def total(items):
    sum_ = 0
    for item in items:
        sum_ += item
    return sum_
```
