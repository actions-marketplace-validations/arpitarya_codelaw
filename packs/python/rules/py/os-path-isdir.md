---
id: py/os-path-isdir
title: "pathlib, not os.path.isdir"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH112]
since: 0.5.0
---

`os.path.isdir()` is written with `pathlib`: `Path.is_dir`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.isdir($P)'
message: "Use pathlib: `Path.is_dir`"
fix: 'pathlib.Path($P).is_dir()'
```

## Examples

### Python

#### Don't

```python
x = os.path.isdir("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").is_dir()
```
