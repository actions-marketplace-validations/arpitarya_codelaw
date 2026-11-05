---
id: py/boolean-chained-comparison
title: "a < b < c, not a < b and b < c"
languages: [python]
fix: auto
fix_reason: "auto when the shared operand is a plain name, so evaluating it once changes nothing"
stability: preview
ported_from: [ruff/PLR1716, pylint/chained-comparison]
since: 0.5.0
---

`a < b and b < c` is written `a < b < c`.

**Why:** The chained form states the range once.

```codelaw
match:
  pattern: $A < $B and $B < $C
constraints:
  B: {kind: identifier}
message: "Write `$A < $B < $C`"
fix: "$A < $B < $C"
```

## Examples

### Python

#### Don't

```python
if 0 < x and x < 10:
    pass
```

#### Do

```python
if 0 < x < 10:
    pass
```
