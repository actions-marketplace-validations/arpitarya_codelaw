---
id: py/unnecessary-round
title: "No round() of an int"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF057]
since: 0.5.0
---

`round()` with no digits is not called on an int literal or on `len()`.

**Why:** The value is already an int.

```codelaw
match:
  pattern: round($X)
constraints:
  X: {any: [{kind: integer}, {regex: '^len\('}]}
message: "`$X` is already an int"
fix: "$X"
```

## Examples

### Python

#### Don't

```python
n = round(len(items))
```

#### Do

```python
n = len(items)
```
