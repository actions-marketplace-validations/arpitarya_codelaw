---
id: py/compare-with-tuple
title: "x in (a, b), not x == a or x == b"
languages: [python]
fix: auto
fix_reason: "auto when the tested value is a plain name"
stability: preview
ported_from: [ruff/SIM109, pylint/consider-using-in]
since: 0.5.0
---

`x == a or x == b` is written `x in (a, b)`.

**Why:** One membership test reads as the list of accepted values.

```codelaw
match:
  pattern: $X == $A or $X == $B
constraints:
  X: {kind: identifier}
message: "Write `$X in ($A, $B)`"
fix: "$X in ($A, $B)"
```

## Examples

### Python

#### Don't

```python
if mode == "a" or mode == "b":
    pass
```

#### Do

```python
if mode in ("a", "b"):
    pass
```
