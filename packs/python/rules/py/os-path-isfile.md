---
id: py/os-path-isfile
title: "pathlib, not os.path.isfile"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH113]
since: 0.5.0
---

`os.path.isfile()` is written with `pathlib`: `Path.is_file`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.isfile($P)'
message: "Use pathlib: `Path.is_file`"
fix: 'pathlib.Path($P).is_file()'
```

## Examples

### Python

#### Don't

```python
x = os.path.isfile("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").is_file()
```
