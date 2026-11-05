---
id: py/os-readlink
title: "pathlib, not os.readlink"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH115]
since: 0.5.0
---

`os.readlink()` is written with `pathlib`: `Path.readlink`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.readlink($P)'
message: "Use pathlib: `Path.readlink`"
fix: 'pathlib.Path($P).readlink()'
```

## Examples

### Python

#### Don't

```python
x = os.readlink("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").readlink()
```
