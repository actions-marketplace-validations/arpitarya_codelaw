---
id: py/type-of-primitive
title: "str, not type('')"
languages: [python]
fix: suggest
fix_reason: "the builtin is the same type; the text changes"
stability: preview
ported_from: [ruff/UP003]
since: 0.5.0
---

`type("")`, `type(0)`, `type(0.0)`, `type(b"")` and `type(True)` are written `str`, `int`, `float`, `bytes` and `bool`.

**Why:** The builtin names the type directly.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^type$}
  - has: {field: arguments, regex: ^\(\s*(""|'')\s*\)$}
message: Write `str`
fix: str
```

## Examples

### Python

#### Don't

```python
if type(x) == type(""):
    pass
```

#### Do

```python
if type(x) == str:
    pass
```
