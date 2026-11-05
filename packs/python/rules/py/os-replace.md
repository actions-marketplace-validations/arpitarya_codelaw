---
id: py/os-replace
title: "pathlib, not os.replace"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH105]
since: 0.5.0
---

`os.replace()` is written with `pathlib`: `Path.replace`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.replace($P, $Q)'
message: "Use pathlib: `Path.replace`"
fix: 'pathlib.Path($P).replace($Q)'
```

## Examples

### Python

#### Don't

```python
x = os.replace("data/a.txt", "data/b.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").replace("data/b.txt")
```
