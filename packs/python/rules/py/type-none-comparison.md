---
id: py/type-none-comparison
title: "x is None, not type(x) is type(None)"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/FURB169]
since: 0.5.0
---

`type(x) is type(None)` is written `x is None`.

**Why:** `None` is the only instance of its type.

```codelaw
match:
  any:
    - pattern: type($X) is type(None)
    - pattern: type($X) == type(None)
message: "Write `$X is None`"
fix: "$X is None"
```

## Examples

### Python

#### Don't

```python
if type(v) is type(None):
    pass
```

#### Do

```python
if v is None:
    pass
```
