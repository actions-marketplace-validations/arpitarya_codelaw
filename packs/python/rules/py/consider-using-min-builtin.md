---
id: py/consider-using-min-builtin
title: "x = min(x, y), not if x > y: x = y"
languages: [python]
fix: suggest
fix_reason: "on equal values `min` keeps the first argument"
stability: preview
ported_from: [pylint/consider-using-min-builtin]
since: 0.5.0
---

`if x > y: x = y` is written `x = min(x, y)`.

**Why:** `min` says what the branch computes.

```codelaw
match:
  kind: if_statement
  pattern: |
    if $X > $Y:
        $X = $Y
  not: {has: {any: [{kind: else_clause}, {kind: elif_clause}]}}
message: "Write `$X = min($X, $Y)`"
fix: "$X = min($X, $Y)"
```

## Examples

### Python

#### Don't

```python
if low > value:
    low = value
```

#### Do

```python
low = min(low, value)
```
