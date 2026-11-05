---
id: py/redundant-log-base
title: "math.log2(x), not math.log(x, 2)"
languages: [python]
fix: suggest
fix_reason: "`log2` is more precise, so results can change in the last digits"
stability: preview
ported_from: [ruff/FURB163]
since: 0.5.0
---

`math.log(x, 2)` is written `math.log2(x)` (and base 10 `math.log10(x)`). The fix covers base 2.

**Why:** The dedicated function is exact for powers of the base.

```codelaw
match:
  pattern: math.log($X, 2)
message: "Write `math.log2($X)`"
fix: "math.log2($X)"
```

## Examples

### Python

#### Don't

```python
bits = math.log(n, 2)
```

#### Do

```python
bits = math.log2(n)
```
