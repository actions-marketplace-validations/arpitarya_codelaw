---
id: py/unnecessary-literal-within-dict-call
title: "No dict() around a dict literal"
languages: [python]
fix: suggest
fix_reason: "the literal is no longer copied; review a literal that is mutated after"
stability: preview
ported_from: [ruff/C418]
since: 0.5.0
---

`dict({...})` is written `{...}`.

**Why:** The outer call copies a dict just built.

```codelaw
match:
  pattern: dict($D)
constraints:
  D: {any: [{kind: dictionary}, {kind: dictionary_comprehension}]}
message: "Drop the `dict()` call"
fix: "$D"
```

## Examples

### Python

#### Don't

```python
cfg = dict({"debug": True})
```

#### Do

```python
cfg = {"debug": True}
```
