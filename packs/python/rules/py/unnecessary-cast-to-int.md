---
id: py/unnecessary-cast-to-int
title: "No int() around what is already an int"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF046]
since: 0.5.0
---

`int()` is not wrapped around `len()`, `round(x)` with no digits, `math.floor`, `math.ceil`, `math.trunc` or `math.isqrt`, which already return an int.

**Why:** The conversion does nothing.

```codelaw
match:
  pattern: int($X)
constraints:
  X: {kind: call, regex: '^(len|math\.(floor|ceil|trunc|isqrt|comb|perm|factorial|gcd|lcm)|ord|hash|round\([^,()]*\))\('}
message: "`$X` is already an int"
fix: "$X"
```

## Examples

### Python

#### Don't

```python
n = int(len(items))
```

#### Do

```python
n = len(items)
```
