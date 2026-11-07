---
id: py/float-equality-comparison
title: "No == between floats"
languages: [python]
fix: suggest
fix_reason: "`math.isclose` uses a tolerance, so values that compared unequal may compare equal; `math` is imported where the file does not, and a file that binds `math` to something else is fixed by hand"
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
  all:
    - any:
        - all: [{ pattern: $A == $B }, { pattern: $EQ }]
        - all: [{ pattern: $A != $B }, { pattern: $NE }]
message: "Compare floats with `math.isclose`"
fix:
  - replace: { target: $EQ, with: "math.isclose($A, $B)" }
  - replace: { target: $NE, with: "not math.isclose($A, $B)" }
  - ensure-import: { module: math }
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
import math

if math.isclose(total, 0.3):
    done()
```
