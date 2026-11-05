---
id: py/os-rename
title: "pathlib, not os.rename"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH104]
since: 0.5.0
---

`os.rename()` is written with `pathlib`: `Path.rename`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.rename($P, $Q)'
message: "Use pathlib: `Path.rename`"
fix: 'pathlib.Path($P).rename($Q)'
```

## Examples

### Python

#### Don't

```python
x = os.rename("data/a.txt", "data/b.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").rename("data/b.txt")
```
