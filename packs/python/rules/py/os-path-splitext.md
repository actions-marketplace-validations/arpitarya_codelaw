---
id: py/os-path-splitext
title: "pathlib, not os.path.splitext"
languages: [python]
fix: manual
fix_reason: "the code moves to `pathlib`, whose objects and methods differ from the strings it used"
fix_exception: restructure
stability: preview
ported_from: [ruff/PTH122]
since: 0.5.0
---

`os.path.splitext(p)` is written with `pathlib`: `Path(p).stem` and `Path(p).suffix`.

**Why:** `pathlib` keeps paths as objects with their own operations, one API instead of several string functions.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^os\.path\.splitext$'}
message: "Use `Path(p).suffix` and `.stem`"
```

## How to fix

1. Rewrite it with `Path(p).suffix` and `.stem`, importing `pathlib.Path`.

## Examples

### Python

#### Don't

```python
root, ext = os.path.splitext(path)
```

#### Do

```python
ext = Path(path).suffix
```
