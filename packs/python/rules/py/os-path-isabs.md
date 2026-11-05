---
id: py/os-path-isabs
title: "pathlib, not os.path.isabs"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH117]
since: 0.5.0
---

`os.path.isabs()` is written with `pathlib`: `Path.is_absolute`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.isabs($P)'
message: "Use pathlib: `Path.is_absolute`"
fix: 'pathlib.Path($P).is_absolute()'
```

## Examples

### Python

#### Don't

```python
x = os.path.isabs("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").is_absolute()
```
