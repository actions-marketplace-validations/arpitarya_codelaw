---
id: py/use-implicit-booleaness-not-len
title: "if x:, not if len(x):"
languages: [python]
fix: suggest
fix_reason: "`bool(x)` is the same as `len(x) != 0` for containers; a type with `__bool__` differing from `__len__` would change"
stability: preview
ported_from: [ruff/PLC1802, pylint/use-implicit-booleaness-not-len]
since: 0.5.0
---

A condition tests a sequence's truth, `if items:`, not `if len(items):`.

**Why:** Empty containers are already falsy.

```codelaw
match:
  pattern: len($X)
  inside: {any: [{kind: if_statement}, {kind: elif_clause}, {kind: while_statement}, {kind: not_operator}]}
message: "Test `$X` directly"
fix: "$X"
```

## Examples

### Python

#### Don't

```python
if len(items):
    go()
```

#### Do

```python
if items:
    go()
```
