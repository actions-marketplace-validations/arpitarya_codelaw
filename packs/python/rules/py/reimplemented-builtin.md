---
id: py/reimplemented-builtin
title: "any(), not a loop that returns True"
languages: [python]
fix: suggest
fix_reason: "the loop and the return after it become one `return any(...)`"
stability: preview
ported_from: [ruff/SIM110]
since: 0.5.0
---

`for x in xs: if cond: return True` followed by `return False` is written `return any(cond for x in xs)`.

**Why:** `any` says what the loop computes.

```codelaw
match:
  kind: for_statement
  pattern: |
    for $X in $XS:
        if $C:
            return True
  precedes: {kind: return_statement, regex: '^return\s+False$', pattern: $RET}
message: "Write `return any($C for $X in $XS)`"
fix:
  - remove: $RET
  - replace: "return any($C for $X in $XS)"
```

## Examples

### Python

#### Don't

```python
def has_neg(xs):
    for x in xs:
        if x < 0:
            return True
    return False
```

#### Do

```python
def has_neg(xs):
    return any(x < 0 for x in xs)
```
