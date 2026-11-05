---
id: py/slice-copy
title: "x.copy(), not x[:]"
languages: [python]
fix: suggest
fix_reason: "the same for a list; a sequence of another type may lack `copy`"
stability: preview
ported_from: [ruff/FURB145]
since: 0.5.0
---

A full-slice copy `x[:]` assigned to a name is written `x.copy()`.

**Why:** `copy` names the operation.

```codelaw
match:
  kind: subscript
  pattern: $X[:]
  inside: {kind: assignment, field: right}
constraints:
  X: {kind: identifier}
message: "Write `$X.copy()`"
fix: "$X.copy()"
```

## Examples

### Python

#### Don't

```python
backup = items[:]
```

#### Do

```python
backup = items.copy()
```
