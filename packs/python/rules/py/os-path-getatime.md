---
id: py/os-path-getatime
title: "pathlib, not os.path.getatime"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH203]
since: 0.5.0
---

`os.path.getatime()` is written with `pathlib`: `Path.stat().st_atime`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.getatime($P)'
message: "Use pathlib: `Path.stat().st_atime`"
fix: 'pathlib.Path($P).stat().st_atime'
```

## Examples

### Python

#### Don't

```python
x = os.path.getatime("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").stat().st_atime
```
