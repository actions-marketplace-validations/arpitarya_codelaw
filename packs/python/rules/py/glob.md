---
id: py/glob
title: "Path.glob, not glob.glob"
languages: [python]
fix: manual
fix_reason: "the code moves to `pathlib`, whose objects and methods differ from the strings it used"
fix_exception: restructure
stability: preview
ported_from: [ruff/PTH207]
since: 0.5.0
---

`glob.glob(pattern)` and `glob.iglob(pattern)` are written with `Path(dir).glob(pattern)`.

**Why:** `pathlib` keeps paths as objects with their own operations, one API instead of several string functions.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^glob\.(glob|iglob)$'}
message: "Use `Path(dir).glob(pattern)`"
```

## How to fix

1. Rewrite it with `Path(dir).glob(pattern)`, importing `pathlib.Path`.

## Examples

### Python

#### Don't

```python
files = glob.glob("*.py")
```

#### Do

```python
files = Path().glob("*.py")
```
