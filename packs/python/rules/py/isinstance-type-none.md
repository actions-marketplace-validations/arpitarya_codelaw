---
id: py/isinstance-type-none
title: "x is None, not isinstance(x, type(None))"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/FURB168]
since: 0.5.0
---

`isinstance(x, type(None))` is written `x is None`.

**Why:** `None` is the only instance of its type.

```codelaw
match:
  pattern: isinstance($X, type(None))
message: "Write `$X is None`"
fix: "$X is None"
```

## Examples

### Python

#### Don't

```python
if isinstance(v, type(None)):
    pass
```

#### Do

```python
if v is None:
    pass
```
