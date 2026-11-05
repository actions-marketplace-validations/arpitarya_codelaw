---
id: py/os-listdir
title: "pathlib, not os.listdir"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH208]
since: 0.5.0
---

`os.listdir()` is written with `pathlib`: `Path.iterdir`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.listdir($P)'
message: "Use pathlib: `Path.iterdir`"
fix: '[p.name for p in pathlib.Path($P).iterdir()]'
```

## Examples

### Python

#### Don't

```python
x = os.listdir("data/a.txt")
```

#### Do

```python
x = [p.name for p in pathlib.Path("data/a.txt").iterdir()]
```
