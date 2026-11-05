---
id: py/os-path-getmtime
title: "pathlib, not os.path.getmtime"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH204]
since: 0.5.0
---

`os.path.getmtime()` is written with `pathlib`: `Path.stat().st_mtime`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.getmtime($P)'
message: "Use pathlib: `Path.stat().st_mtime`"
fix: 'pathlib.Path($P).stat().st_mtime'
```

## Examples

### Python

#### Don't

```python
x = os.path.getmtime("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").stat().st_mtime
```
