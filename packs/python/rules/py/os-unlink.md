---
id: py/os-unlink
title: "pathlib, not os.unlink"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH108]
since: 0.5.0
---

`os.unlink()` is written with `pathlib`: `Path.unlink`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.unlink($P)'
message: "Use pathlib: `Path.unlink`"
fix: 'pathlib.Path($P).unlink()'
```

## Examples

### Python

#### Don't

```python
x = os.unlink("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").unlink()
```
