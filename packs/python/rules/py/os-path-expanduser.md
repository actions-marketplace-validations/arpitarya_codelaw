---
id: py/os-path-expanduser
title: "pathlib, not os.path.expanduser"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH111]
since: 0.5.0
---

`os.path.expanduser()` is written with `pathlib`: `Path.expanduser`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.expanduser($P)'
message: "Use pathlib: `Path.expanduser`"
fix: 'pathlib.Path($P).expanduser()'
```

## Examples

### Python

#### Don't

```python
x = os.path.expanduser("data/a.txt")
```

#### Do

```python
x = pathlib.Path("data/a.txt").expanduser()
```
