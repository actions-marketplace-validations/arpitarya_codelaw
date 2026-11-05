---
id: py/os-remove
title: "pathlib, not os.remove"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH107]
since: 0.5.0
---

`os.remove()` is written with `pathlib`: `Path.unlink`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.remove($P)'
message: "Use pathlib: `Path.unlink`"
fix: 'pathlib.Path($P).unlink()'
```

## Examples

### Python

#### Don't

```python
x = os.remove("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").unlink()
```
