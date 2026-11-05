---
id: py/os-getcwd
title: "pathlib, not os.getcwd"
languages: [python]
fix: suggest
fix_reason: "the result is a `Path` (or uses one), and `pathlib` must be imported; review code that expects a string"
stability: preview
ported_from: [ruff/PTH109]
since: 0.5.0
---

`os.getcwd()` is written with `pathlib`: `Path.cwd`.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.getcwd()'
message: "Use pathlib: `Path.cwd`"
fix: 'pathlib.Path.cwd()'
```

## Examples

### Python

#### Don't

```python
x = os.getcwd()
```

#### Do

```python
x = pathlib.Path.cwd()
```
