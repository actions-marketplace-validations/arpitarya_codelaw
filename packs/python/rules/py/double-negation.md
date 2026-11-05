---
id: py/double-negation
title: "No not not"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/SIM208]
since: 0.5.0
---

`not not x` is written `bool(x)`.

**Why:** Two negations cancel; `bool` says the conversion is the point.

```codelaw
match:
  kind: not_operator
  pattern: not not $X
message: "Write `bool($X)`"
fix: "bool($X)"
```

## Examples

### Python

#### Don't

```python
flag = not not value
```

#### Do

```python
flag = bool(value)
```
