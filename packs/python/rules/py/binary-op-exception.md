---
id: py/binary-op-exception
title: "except (A, B), not except A or B"
languages: [python]
fix: suggest
fix_reason: "the handler then also catches `B`, which `A or B` never did"
stability: preview
ported_from: [ruff/PLW0711]
since: 0.5.0
---

Several exception types are caught with a tuple, not with `or` or `and`.

**Why:** `A or B` evaluates to `A`, so `B` is never caught.

```codelaw
match:
  kind: boolean_operator
  pattern: $A or $B
  inside: {kind: except_clause}
message: "`$A or $B` is just `$A`; catch `($A, $B)`"
fix: "($A, $B)"
```

## Examples

### Python

#### Don't

```python
try:
    go()
except ValueError or TypeError:
    pass
```

#### Do

```python
try:
    go()
except (ValueError, TypeError):
    pass
```
