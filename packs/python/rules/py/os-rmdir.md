---
id: py/os-rmdir
title: "pathlib, not os.rmdir"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH106]
since: 0.5.0
---

`os.rmdir()` is written with `pathlib`: `Path.rmdir`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.rmdir($P)'
message: "Use pathlib: `Path.rmdir`"
fix: 'pathlib.Path($P).rmdir()'
```

## Examples

### Python

#### Don't

```python
x = os.rmdir("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").rmdir()
```
