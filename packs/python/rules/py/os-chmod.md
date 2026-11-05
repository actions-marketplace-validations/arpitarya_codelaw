---
id: py/os-chmod
title: "pathlib, not os.chmod"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH101]
since: 0.5.0
---

`os.chmod()` is written with `pathlib`: `Path.chmod`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.chmod($P, $M)'
message: "Use pathlib: `Path.chmod`"
fix: 'pathlib.Path($P).chmod($M)'
```

## Examples

### Python

#### Don't

```python
x = os.chmod("data/a.txt", 0o644)
```

#### Do

```python
x = pathlib.Path("data/a.txt").chmod(0o644)
```
