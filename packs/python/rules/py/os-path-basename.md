---
id: py/os-path-basename
title: "pathlib, not os.path.basename"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH119]
since: 0.5.0
---

`os.path.basename()` is written with `pathlib`: `Path.name`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.basename($P)'
message: "Use pathlib: `Path.name`"
fix: 'pathlib.Path($P).name'
```

## Examples

### Python

#### Don't

```python
x = os.path.basename("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").name
```
