---
id: py/repeated-equality-comparison
title: "x in (a, b), not x == a or x == b"
languages: [python]
fix: auto
fix_reason: "auto when the tested value is a plain name"
stability: preview
ported_from: [ruff/PLR1714]
since: 0.5.0
---

`x == a or x == b or x == c` is written `x in (a, b, c)`. The fix covers three alternatives (`py/compare-with-tuple` covers two).

**Why:** One membership test lists the accepted values.

```codelaw
match:
  pattern: $X == $A or $X == $B or $X == $C
constraints:
  X: {kind: identifier}
message: "Write `$X in ($A, $B, $C)`"
fix: "$X in ($A, $B, $C)"
```

## Examples

### Python

#### Don't

```python
if c == "a" or c == "b" or c == "c":
    pass
```

#### Do

```python
if c in ("a", "b", "c"):
    pass
```
