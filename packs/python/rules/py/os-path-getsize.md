---
id: py/os-path-getsize
title: "pathlib, not os.path.getsize"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH202]
since: 0.5.0
---

`os.path.getsize()` is written with `pathlib`: `Path.stat().st_size`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.getsize($P)'
message: "Use pathlib: `Path.stat().st_size`"
fix: 'pathlib.Path($P).stat().st_size'
```

## Examples

### Python

#### Don't

```python
x = os.path.getsize("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").stat().st_size
```
