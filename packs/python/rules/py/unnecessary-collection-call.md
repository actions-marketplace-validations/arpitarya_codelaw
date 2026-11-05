---
id: py/unnecessary-collection-call
title: "{} , not dict()"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C408, pylint/use-dict-literal]
since: 0.5.0
---

An empty dict is written `{}`, not `dict()`. (Ruff also rewrites `list()`, `tuple()` and `dict(a=1)`; those are left out.)

**Why:** The literal is faster and reads as the value it is.

```codelaw
match:
  pattern: dict()
message: "Write `{}`"
fix: "{}"
```

## Examples

### Python

#### Don't

```python
cache = dict()
```

#### Do

```python
cache = {}
```
