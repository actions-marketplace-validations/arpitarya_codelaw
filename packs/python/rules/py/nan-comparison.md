---
id: py/nan-comparison
title: "math.isnan, not == nan"
languages: [python]
fix: suggest
fix_reason: "the result changes from always False to the NaN test that was meant"
stability: preview
ported_from: [ruff/PLW0177]
since: 0.5.0
---

`x == math.nan` is written `math.isnan(x)` (Ruff also reports `!=`, `float("nan")` and `numpy.nan`; those are left out).

**Why:** NaN is not equal to anything, itself included: `==` is always False.

```codelaw
match:
  kind: comparison_operator
  pattern: $X == math.nan
message: "NaN never compares equal; use `math.isnan($X)`"
fix: "math.isnan($X)"
```

## Examples

### Python

#### Don't

```python
if x == math.nan:
    pass
```

#### Do

```python
if math.isnan(x):
    pass
```
