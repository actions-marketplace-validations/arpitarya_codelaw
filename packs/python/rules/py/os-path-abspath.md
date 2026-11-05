---
id: py/os-path-abspath
title: "pathlib, not os.path.abspath"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH100]
since: 0.5.0
---

`os.path.abspath()` is written with `pathlib`: `Path.resolve`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.abspath($P)'
message: "Use pathlib: `Path.resolve`"
fix: 'pathlib.Path($P).resolve()'
```

## Examples

### Python

#### Don't

```python
x = os.path.abspath("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").resolve()
```
