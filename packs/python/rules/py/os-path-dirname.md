---
id: py/os-path-dirname
title: "pathlib, not os.path.dirname"
languages: [python]
fix: suggest
fix_reason: "`pathlib` must be imported; the string is built the same way for ordinary paths"
stability: preview
ported_from: [ruff/PTH120]
since: 0.5.0
---

`os.path.dirname()` is written with `pathlib`: `Path.parent`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.dirname($P)'
message: "Use pathlib: `Path.parent`"
fix: 'str(pathlib.Path($P).parent)'
```

## Examples

### Python

#### Don't

```python
x = os.path.dirname("data/a.txt")
```

#### Do

```python
x = str(pathlib.Path("data/a.txt").parent)
```
