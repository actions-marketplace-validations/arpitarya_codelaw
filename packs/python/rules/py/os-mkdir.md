---
id: py/os-mkdir
title: "pathlib, not os.mkdir"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH102]
since: 0.5.0
---

`os.mkdir()` is written with `pathlib`: `Path.mkdir`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.mkdir($P)'
message: "Use pathlib: `Path.mkdir`"
fix: 'pathlib.Path($P).mkdir()'
```

## Examples

### Python

#### Don't

```python
x = os.mkdir("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").mkdir()
```
