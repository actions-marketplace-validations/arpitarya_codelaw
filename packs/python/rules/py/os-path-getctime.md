---
id: py/os-path-getctime
title: "pathlib, not os.path.getctime"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH205]
since: 0.5.0
---

`os.path.getctime()` is written with `pathlib`: `Path.stat().st_ctime`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.getctime($P)'
message: "Use pathlib: `Path.stat().st_ctime`"
fix: 'pathlib.Path($P).stat().st_ctime'
```

## Examples

### Python

#### Don't

```python
x = os.path.getctime("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").stat().st_ctime
```
