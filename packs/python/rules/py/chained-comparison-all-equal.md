---
id: py/chained-comparison-all-equal
title: "a == b == c, not a == b and b == c"
languages: [python]
fix: auto
fix_reason: "auto when the shared operand is a plain name, so evaluating it once changes nothing"
stability: preview
ported_from: [pylint/chained-comparison-all-equal]
since: 0.5.0
---

`a == b and b == c` is written `a == b == c`.

**Why:** The chained form states the equality once.

```codelaw
match:
  pattern: $A == $B and $B == $C
constraints:
  B: {kind: identifier}
message: "Write `$A == $B == $C`"
fix: "$A == $B == $C"
```

## Examples

### Python

#### Don't

```python
if x == y and y == z:
    pass
```

#### Do

```python
if x == y == z:
    pass
```
