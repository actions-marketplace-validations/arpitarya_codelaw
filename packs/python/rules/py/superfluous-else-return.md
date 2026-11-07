---
id: py/superfluous-else-return
title: "No else after return"
languages: [python]
fix: manual
fix_reason: "the `else` (or `elif`) is removed and its body dedented, which the engine cannot re-indent in Python yet"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [ruff/RET505, pylint/no-else-return]
since: 0.5.0
---

An `else` or `elif` that follows an `if` branch ending in `return` — every branch before it ending so — is not needed: its body can follow the `if` unindented.

**Why:** After `return` the code below only runs when the branch did not; the `else` adds a level of nesting for nothing.

```codelaw
utils:
  ends:
    kind: block
    has: {nthChild: {position: 1, reverse: true}, kind: return_statement}
match:
  any: [{kind: else_clause}, {kind: elif_clause}]
  inside:
    kind: if_statement
    all:
      - has: {field: consequence, matches: ends}
      - not: {has: {kind: elif_clause, not: {has: {field: consequence, matches: ends}}}}
  regex: ''
message: "Remove the `else`; the `return` already leaves the branch"
```

## How to fix

1. Delete the `else:` (or turn the `elif` into an `if`) and dedent its body one level.

## Examples

### Python

#### Don't

```python
def sign(x):
    if x < 0:
        return -1
    else:
        return 1
```

#### Do

```python
def sign(x):
    if x < 0:
        return -1
    return 1
```
