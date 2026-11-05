---
id: py/if-expr-min-max
title: "min()/max(), not a conditional"
languages: [python]
fix: suggest
fix_reason: "on equal values `min`/`max` return the first argument, where the conditional may return the second"
stability: preview
ported_from: [ruff/FURB136]
since: 0.5.0
---

`a if a > b else b` is written `max(a, b)`. (The `min` forms are left out.)

**Why:** `max` names the comparison.

```codelaw
match:
  pattern: $A if $A > $B else $B
message: "Write `max($A, $B)`"
fix: "max($A, $B)"
```

## Examples

### Python

#### Don't

```python
top = x if x > y else y
```

#### Do

```python
top = max(x, y)
```
