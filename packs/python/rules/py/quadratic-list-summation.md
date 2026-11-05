---
id: py/quadratic-list-summation
title: "No sum() over lists"
languages: [python]
fix: suggest
fix_reason: "the comprehension requires every item to be iterable where `sum` required `+`"
stability: preview
ported_from: [ruff/RUF017]
since: 0.5.0
---

Lists are flattened with a comprehension or `itertools.chain`, not `sum(lists, [])`.

**Why:** `sum` builds a new list at every step, so flattening n lists is quadratic.

```codelaw
match:
  any:
    - pattern: sum($L, [])
    - pattern: sum($L, start=[])
message: "`sum(..., [])` is quadratic; flatten with a comprehension"
fix: "[item for sub in $L for item in sub]"
```

## Examples

### Python

#### Don't

```python
flat = sum(groups, [])
```

#### Do

```python
flat = [item for sub in groups for item in sub]
```
