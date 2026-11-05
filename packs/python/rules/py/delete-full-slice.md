---
id: py/delete-full-slice
title: "x.clear(), not del x[:]"
languages: [python]
fix: suggest
fix_reason: "the same for a list or dict; another type may lack `clear`"
stability: preview
ported_from: [ruff/FURB131]
since: 0.5.0
---

`del items[:]` is written `items.clear()`.

**Why:** `clear` names the operation.

```codelaw
match:
  kind: delete_statement
  pattern: del $X[:]
constraints:
  X: {any: [{kind: identifier}, {kind: attribute}]}
message: "Write `$X.clear()`"
fix: "$X.clear()"
```

## Examples

### Python

#### Don't

```python
del items[:]
```

#### Do

```python
items.clear()
```
