---
id: py/os-stat
title: "pathlib, not os.stat"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH116]
since: 0.5.0
---

`os.stat()` is written with `pathlib`: `Path.stat`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.stat($P)'
message: "Use pathlib: `Path.stat`"
fix: 'pathlib.Path($P).stat()'
```

## Examples

### Python

#### Don't

```python
x = os.stat("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").stat()
```
