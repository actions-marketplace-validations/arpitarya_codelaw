---
id: py/os-path-join
title: "pathlib, not os.path.join"
languages: [python]
fix: suggest
fix_reason: "`pathlib` must be imported; the string is built the same way for ordinary paths"
stability: preview
ported_from: [ruff/PTH118]
since: 0.5.0
---

`os.path.join()` is written with `pathlib`: the `/` operator.

**Why:** `pathlib` keeps paths as objects, and its methods compose; mixing the two styles makes path code harder to read.

```codelaw
match:
  pattern: 'os.path.join($P, $Q)'
message: "Use pathlib: the `/` operator"
fix: 'str(pathlib.Path($P) / $Q)'
```

## Examples

### Python

#### Don't

```python
x = os.path.join("data/a.txt", "data/b.txt")
```

#### Do

```python
x = str(pathlib.Path("data/a.txt") / "data/b.txt")
```
