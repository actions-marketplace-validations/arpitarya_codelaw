---
id: py/os-path-exists
title: "pathlib, not os.path.exists"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH110]
since: 0.5.0
---

`os.path.exists()` is written with `pathlib`: `Path.exists`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.exists($P)'
message: "Use pathlib: `Path.exists`"
fix: 'pathlib.Path($P).exists()'
```

## Examples

### Python

#### Don't

```python
x = os.path.exists("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").exists()
```
