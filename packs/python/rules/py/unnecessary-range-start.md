---
id: py/unnecessary-range-start
title: "range(n), not range(0, n)"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PIE808]
since: 0.5.0
---

`range(0, n)` is written `range(n)`.

**Why:** 0 is the default start.

```codelaw
match:
  pattern: range(0, $N)
constraints:
  N: {not: {any: [{kind: keyword_argument}, {kind: list_splat}]}}
message: "Drop the `0` start"
fix: "range($N)"
```

## Examples

### Python

#### Don't

```python
for i in range(0, 10):
    pass
```

#### Do

```python
for i in range(10):
    pass
```
