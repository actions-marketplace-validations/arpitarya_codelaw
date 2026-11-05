---
id: py/negate-not-equal-op
title: "a == b, not not a != b"
languages: [python]
fix: suggest
fix_reason: "a class may define `__eq__` differently from `not __ne__`"
stability: preview
ported_from: [ruff/SIM202]
since: 0.5.0
---

`not a != b` is written `a == b`.

**Why:** The operator says it directly.

```codelaw
match:
  kind: not_operator
  pattern: not $A != $B
message: "Write `$A == $B`"
fix: "$A == $B"
```

## Examples

### Python

#### Don't

```python
if not x != 1:
    pass
```

#### Do

```python
if x == 1:
    pass
```
