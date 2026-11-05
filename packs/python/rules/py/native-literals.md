---
id: py/native-literals
title: "Literals, not str() or int() of a literal"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP018]
since: 0.5.0
---

`str("x")`, `int(1)`, `float(1.0)`, `bool(True)` and `bytes(b"x")` are written as the literal; `str()` as `""`.

**Why:** The call converts a value to the type it already has.

```codelaw
match:
  any:
    - pattern: str($L)
    - pattern: int($L)
    - pattern: float($L)
  has: {field: function, pattern: $F}
constraints:
  L: {any: [{kind: string, not: {regex: '^[rR]?[bBfF]'}}, {kind: integer}, {kind: float}]}
message: "`$L` is already the right type"
fix: "$L"
```

## Examples

### Python

#### Don't

```python
name = str("bob")
```

#### Do

```python
name = "bob"
```
