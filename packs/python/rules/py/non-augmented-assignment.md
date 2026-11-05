---
id: py/non-augmented-assignment
title: "x += 1, not x = x + 1"
languages: [python]
fix: suggest
fix_reason: "`+=` mutates a mutable left operand in place, which other references see"
stability: preview
ported_from: [ruff/PLR6104, pylint/consider-using-augmented-assign]
since: 0.5.0
---

`x = x + y` (and `-`, `*`, `/`, `//`, `%`, `**`, `|`, `&`, `^`, `<<`, `>>`) is written `x += y`. The fix covers `+`.

**Why:** The augmented form names the target once.

```codelaw
match:
  kind: assignment
  pattern: $X = $X + $Y
  not: {has: {field: type, kind: type}}
constraints:
  X: {any: [{kind: identifier}, {kind: attribute}]}
message: "Write `$X += $Y`"
fix: "$X += $Y"
```

## Examples

### Python

#### Don't

```python
count = count + 1
```

#### Do

```python
count += 1
```
