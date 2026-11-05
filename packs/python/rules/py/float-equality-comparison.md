---
id: py/float-equality-comparison
title: "No == between floats"
languages: [python]
fix: manual
fix_reason: "`math.isclose` uses a tolerance, so values that compared unequal may compare equal"
fix_pending: "suggest; needs add-import"
stability: preview
ported_from: [ruff/RUF069]
since: 0.5.0
---

A float literal is not compared with `==` or `!=`.

**Why:** Floating-point results carry rounding error; `0.1 + 0.2 == 0.3` is False.

```codelaw
match:
  kind: comparison_operator
  any:
    - pattern: $A == $B
    - pattern: $A != $B
  has: {kind: float}
message: "Compare floats with `math.isclose`"
```

## How to fix

1. Use `math.isclose(a, b)` (or `not math.isclose(...)` for `!=`).

## Examples

### Python

#### Don't

```python
if total == 0.3:
    done()
```

#### Do

```python
if math.isclose(total, 0.3):
    done()
```
