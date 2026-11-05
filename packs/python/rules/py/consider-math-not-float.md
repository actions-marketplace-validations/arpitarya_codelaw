---
id: py/consider-math-not-float
title: "math.inf, not float('inf')"
languages: [python]
fix: suggest
fix_reason: "`math` must be imported"
stability: preview
ported_from: [pylint/consider-math-not-float]
since: 0.5.0
---

`float("inf")` and `float("nan")` are written `math.inf` and `math.nan`. The fix covers `inf`.

**Why:** The constant needs no string parse and reads as the value.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^float$}
  - has: {field: arguments, regex: '^\(\s*["''](inf|INF|Inf)["'']\s*\)$'}
message: Write `math.inf`
fix: math.inf
```

## Examples

### Python

#### Don't

```python
best = float("inf")
```

#### Do

```python
best = math.inf
```
