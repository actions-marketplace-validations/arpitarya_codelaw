---
id: py/bit-count
title: "int.bit_count(), not bin(x).count('1')"
languages: [python]
fix: suggest
fix_reason: "`bit_count` needs Python 3.10 and an int"
stability: preview
ported_from: [ruff/FURB161]
since: 0.5.0
---

`bin(x).count("1")` is written `x.bit_count()` (Python 3.10+).

**Why:** `bit_count` counts set bits without building a string.

```codelaw
match:
  pattern: bin($X).count("1")
constraints:
  X: {kind: identifier}
message: "Write `$X.bit_count()`"
fix: "$X.bit_count()"
```

## Examples

### Python

#### Don't

```python
n = bin(mask).count("1")
```

#### Do

```python
n = mask.bit_count()
```
