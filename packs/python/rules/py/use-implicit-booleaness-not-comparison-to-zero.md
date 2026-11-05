---
id: py/use-implicit-booleaness-not-comparison-to-zero
title: "if not x:, not if x == 0"
languages: [python]
fix: suggest
fix_reason: "`not x` is also true for `None` and other falsy values"
stability: preview
ported_from: [pylint/use-implicit-booleaness-not-comparison-to-zero]
since: 0.5.0
---

In a condition, `x == 0` is written `not x` (Pylint's optional check).

**Why:** Zero is already falsy.

```codelaw
match:
  kind: comparison_operator
  pattern: $X == 0
  inside: {any: [{kind: if_statement}, {kind: elif_clause}, {kind: while_statement}]}
constraints:
  X: {any: [{kind: identifier}, {kind: attribute}]}
message: "Write `not $X`"
fix: "not $X"
```

## Examples

### Python

#### Don't

```python
if count == 0:
    stop()
```

#### Do

```python
if not count:
    stop()
```
