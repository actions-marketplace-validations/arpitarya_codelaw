---
id: py/if-stmt-min-max
title: "x = max(x, y), not if x < y: x = y"
languages: [python]
fix: suggest
fix_reason: "on equal values `max` keeps the first argument"
stability: preview
ported_from: [ruff/PLR1730, pylint/consider-using-max-builtin]
since: 0.5.0
---

`if x < y: x = y` is written `x = max(x, y)` (and `>` with `min`). The fix covers the `max` form.

**Why:** `max` says what the branch computes.

```codelaw
match:
  kind: if_statement
  pattern: |
    if $X < $Y:
        $X = $Y
  not: {has: {any: [{kind: else_clause}, {kind: elif_clause}]}}
message: "Write `$X = max($X, $Y)`"
fix: "$X = max($X, $Y)"
```

## Examples

### Python

#### Don't

```python
if best < score:
    best = score
```

#### Do

```python
best = max(best, score)
```
