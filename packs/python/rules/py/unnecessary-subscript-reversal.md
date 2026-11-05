---
id: py/unnecessary-subscript-reversal
title: "No [::-1] inside set() or sorted()"
languages: [python]
fix: auto
fix_reason: "auto for `set`, whose result does not depend on order"
stability: preview
ported_from: [ruff/C415]
since: 0.5.0
---

`set(x[::-1])` is written `set(x)`. (Ruff also reports `sorted` and `reversed`; those are left out.)

**Why:** Reversing first changes nothing about the set.

```codelaw
match:
  pattern: set($X[::-1])
message: "The reversal changes nothing; write `set($X)`"
fix: "set($X)"
```

## Examples

### Python

#### Don't

```python
unique = set(items[::-1])
```

#### Do

```python
unique = set(items)
```
