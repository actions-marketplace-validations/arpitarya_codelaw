---
id: py/os-makedirs
title: "pathlib, not os.makedirs"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH103]
since: 0.5.0
---

`os.makedirs()` is written with `pathlib`: `Path.mkdir(parents=True)`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.makedirs($P)'
message: "Use pathlib: `Path.mkdir(parents=True)`"
fix: 'pathlib.Path($P).mkdir(parents=True)'
```

## Examples

### Python

#### Don't

```python
x = os.makedirs("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").mkdir(parents=True)
```
