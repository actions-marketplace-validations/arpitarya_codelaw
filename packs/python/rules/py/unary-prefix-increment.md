---
id: py/unary-prefix-increment
title: "No `++x` or `--x`"
languages: [python]
fix: suggest
fix_reason: "as a statement the choices are `x += 1` (`-=` for `--`) and deleting it; inside an expression the two signs go. `++x` is a no-op today, but `__pos__` may not be (Decimal), so none is automatic or the default"
stability: preview
ported_from: [ruff/B002, pylint/nonexistent-operator]
since: 0.1.0
---

Code doesn't write `++x` or `--x`.

**Why:** Python has no increment operator: `++x` is `+(+x)`, which leaves `x` unchanged.

```codelaw
native: py/unary-prefix-increment
```

## How to fix

1. Write `x += 1` (or `x -= 1`) if an increment was meant.

## Examples

### Python

#### Don't

```py
count = 1
++count
```

#### Fixed (increment)

```py
count = 1
count += 1
```

#### Fixed (delete)

```py
count = 1
```

#### Do

```py
count = 1
count += 1
```
