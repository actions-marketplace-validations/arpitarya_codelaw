---
id: py/decimal-from-float-literal
title: "Decimal from a string, not a float"
languages: [python]
fix: suggest
fix_reason: "the value changes: the string is the decimal the author wrote, the float is its binary approximation"
stability: preview
ported_from: [ruff/RUF032]
since: 0.5.0
---

`Decimal(0.1)` is written `Decimal("0.1")`.

**Why:** A float literal is converted through binary floating point first, so `Decimal(0.1)` is not one tenth.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^(decimal\.)?Decimal$'}
    - has: {field: arguments, has: {kind: float, nthChild: 1, pattern: $F}, not: {has: {nthChild: 2}}}
message: "`Decimal($F)` carries the float's binary error; pass `\"$F\"`"
fix:
  - replace: {target: $F, with: '"$F"'}
```

## Examples

### Python

#### Don't

```python
price = Decimal(0.1)
```

#### Do

```python
price = Decimal("0.1")
```
