---
id: py/comparison-with-itself
title: "No comparison of a name with itself"
languages: [python]
fix: manual
fix_reason: "what the comparison was meant to test is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLR0124, pylint/comparison-with-itself]
since: 0.5.0
---

A name or attribute is not compared with itself: `x == x`, `x is x`, `x < x`.

**Why:** The result is fixed (or, for `x != x`, the hidden NaN test `math.isnan` says better).

```codelaw
match:
  kind: comparison_operator
  any:
    - pattern: $A == $A
    - pattern: $A != $A
    - pattern: $A < $A
    - pattern: $A > $A
    - pattern: $A <= $A
    - pattern: $A >= $A
    - pattern: $A is $A
    - pattern: $A is not $A
constraints:
  A: {any: [{kind: identifier}, {kind: attribute}]}
message: "`$A` is compared with itself"
```

## How to fix

1. Compare with the value that was meant, or use `math.isnan(x)` for a NaN test.

## Examples

### Python

#### Don't

```python
if total == total:
    pass
```

#### Do

```python
if total == expected:
    pass
```
