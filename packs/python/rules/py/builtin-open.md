---
id: py/builtin-open
title: "Path.open, not open()"
languages: [python]
fix: suggest
fix_reason: "`pathlib` must be imported"
stability: preview
ported_from: [ruff/PTH123]
since: 0.5.0
---

`open(path)` with a path literal is written `pathlib.Path(path).open()`.

**Why:** `pathlib` keeps paths as objects.

```codelaw
match:
  pattern: open($P)
constraints:
  P: {kind: string}
message: "Use `pathlib.Path($P).open()`"
fix: 'pathlib.Path($P).open()'
```

## Examples

### Python

#### Don't

```python
f = open("notes.txt")
```

#### Do

```python
f = pathlib.Path("notes.txt").open()
```
