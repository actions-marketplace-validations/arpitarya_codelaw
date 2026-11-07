---
id: py/py-path
title: "pathlib, not py.path"
languages: [python]
fix: manual
fix_reason: "the code moves to `pathlib`, whose objects and methods differ from the strings it used"
fix_exception: restructure
stability: preview
ported_from: [ruff/PTH124]
since: 0.5.0
---

`py.path.local(...)`, from the `py` library, is written with `pathlib.Path`.

**Why:** `pathlib` keeps paths as objects with their own operations, one API instead of several string functions.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^py\.path\.local$'}
message: "Use `pathlib.Path`"
```

## How to fix

1. Rewrite it with `pathlib.Path`, importing `pathlib.Path`.

## Examples

### Python

#### Don't

```python
p = py.path.local(tmpdir)
```

#### Do

```python
p = Path(tmpdir)
```
