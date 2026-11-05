---
id: py/verbose-decimal-constructor
title: "Decimal('1.5'), not Decimal('  1.5 ')"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/FURB157]
since: 0.5.0
---

`Decimal("5")` with an integer string is written `Decimal(5)`.

**Why:** An int converts exactly; the string adds a parse.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: '^(decimal\.)?Decimal$', pattern: $F}
  - has:
      field: arguments
      regex: ^\(\s*["'][0-9]+["']\s*\)$
      has:
        kind: string
        has: {kind: string_content, pattern: $N}
message: Write `$F($N)`
fix: $F($N)
```

## Examples

### Python

#### Don't

```python
price = Decimal("5")
```

#### Do

```python
price = Decimal(5)
```
