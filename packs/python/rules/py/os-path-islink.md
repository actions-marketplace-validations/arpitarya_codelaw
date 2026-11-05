---
id: py/os-path-islink
title: "pathlib, not os.path.islink"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH114]
since: 0.5.0
---

`os.path.islink()` is written with `pathlib`: `Path.is_symlink`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.islink($P)'
message: "Use pathlib: `Path.is_symlink`"
fix: 'pathlib.Path($P).is_symlink()'
```

## Examples

### Python

#### Don't

```python
x = os.path.islink("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").is_symlink()
```
