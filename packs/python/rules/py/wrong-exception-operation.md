---
id: py/wrong-exception-operation
title: "An except names exceptions, not an expression of them"
languages: [python]
fix: manual
fix_reason: "which exceptions were meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/wrong-exception-operation]
since: 0.5.0
---

An `except` names an exception or a tuple of them, not `A or B` or `A + B`.

**Why:** `A or B` is just `A`, so `B` is never caught.

```codelaw
match:
  all:
    - any:
        - kind: boolean_operator
        - kind: binary_operator
    - any:
        - inside: {kind: except_clause}
        - inside: {kind: parenthesized_expression, inside: {kind: except_clause}}
        - inside: {kind: as_pattern, inside: {kind: except_clause}}
message: "Write the exceptions as a tuple"
```

## How to fix

1. List the exceptions in a tuple: `except (A, B):`.

## Examples

### Python

#### Don't

```python
try:
    run()
except (KeyError or IndexError):
    pass
```

#### Do

```python
try:
    run()
except (KeyError, IndexError):
    pass
```
