---
id: py/yoda-conditions
title: "The variable goes first in a comparison"
languages: [python]
fix: suggest
fix_reason: "swapping the operands calls the other side's `__eq__` first"
stability: preview
ported_from: [ruff/SIM300, pylint/misplaced-comparison-constant]
since: 0.5.0
---

`"a" == x` is written `x == "a"` (for `==`; Ruff also flips `!=`, `<` and `>`, which are left out).

**Why:** Code reads as the thing being tested, then the value it is tested against.

```codelaw
match:
  kind: comparison_operator
  pattern: $L == $R
constraints:
  L: {not: {regex: '^[rRbB]?[fF]'}, any: [{kind: string}, {kind: integer}, {kind: float}, {kind: none}, {kind: 'true'}, {kind: 'false'}]}
  R: {any: [{kind: identifier}, {kind: attribute}, {kind: call}, {kind: subscript}]}
message: "Put `$R` first"
fix: "$R == $L"
```

## Examples

### Python

#### Don't

```python
if "admin" == role:
    pass
```

#### Do

```python
if role == "admin":
    pass
```
