---
id: py/math-constant
title: "math.pi, not 3.14159"
languages: [python]
fix: suggest
fix_reason: "the constant is more precise than the literal, so results can change in the last digits"
stability: preview
ported_from: [ruff/FURB152]
since: 0.5.0
---

A literal that approximates π, τ or e to at least two decimals (`3.14`, `6.28`, `2.71`) is written `math.pi`, `math.tau` or `math.e`. The fix covers π.

**Why:** The constant is exact to double precision and says what the number is.

```codelaw
match:
  kind: float
  regex: '^3\.14\d*$'
message: "Use `math.pi`"
fix: "math.pi"
```

## Examples

### Python

#### Don't

```python
area = 3.14159 * r * r
```

#### Do

```python
area = math.pi * r * r
```
