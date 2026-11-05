---
id: py/unnecessary-from-float
title: "Decimal(x), not Decimal.from_float(x)"
languages: [python]
fix: suggest
fix_reason: "the constructor accepts more types than `from_float`"
stability: preview
ported_from: [ruff/FURB164]
since: 0.5.0
---

`Decimal.from_float(x)` and `Fraction.from_float(x)` are written `Decimal(x)` and `Fraction(x)`.

**Why:** The constructor does the same conversion.

```codelaw
match:
  pattern: $C.from_float($X)
constraints:
  C: {regex: '^(Decimal|Fraction)$'}
message: "Write `$C($X)`"
fix: "$C($X)"
```

## Examples

### Python

#### Don't

```python
d = Decimal.from_float(x)
```

#### Do

```python
d = Decimal(x)
```
