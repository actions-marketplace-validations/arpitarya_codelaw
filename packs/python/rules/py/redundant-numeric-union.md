---
id: py/redundant-numeric-union
title: "float, not int | float"
languages: [python]
fix: suggest
fix_reason: "the annotation text changes; the accepted values are the same"
stability: preview
ported_from: [ruff/PYI041]
since: 0.5.0
---

`int | float` is written `float`, and `float | complex` `complex`.

**Why:** Type checkers already accept an `int` where `float` is annotated.

```codelaw
match:
  kind: binary_operator
  pattern: int | float
message: "Write `float`"
fix: "float"
```

## Examples

### Python

#### Don't

```python
def f(x: int | float) -> None: ...
```

#### Do

```python
def f(x: float) -> None: ...
```
