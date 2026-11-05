---
id: py/os-symlink
title: "pathlib, not os.symlink"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH211]
since: 0.5.0
---

`os.symlink()` is written with `pathlib`: `Path.symlink_to`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.symlink($P, $Q)'
message: "Use pathlib: `Path.symlink_to`"
fix: 'pathlib.Path($Q).symlink_to($P)'
```

## Examples

### Python

#### Don't

```python
x = os.symlink("data/a.txt", "data/b.txt")
```

#### Do

```python
x = pathlib.Path("data/b.txt").symlink_to("data/a.txt")
```
