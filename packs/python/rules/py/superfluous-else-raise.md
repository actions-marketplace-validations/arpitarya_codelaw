---
id: py/superfluous-else-raise
title: "No else after raise"
languages: [python]
fix: manual
fix_reason: "the `else` (or `elif`) is removed and its body dedented, which the engine cannot re-indent in Python yet"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [ruff/RET506, pylint/no-else-raise]
since: 0.5.0
---

An `else` or `elif` that follows an `if` branch ending in `raise` — every branch before it ending so — is not needed: its body can follow the `if` unindented.

**Why:** After `raise` the code below only runs when the branch did not; the `else` adds a level of nesting for nothing.

```codelaw
utils:
  ends:
    kind: block
    has: {nthChild: {position: 1, reverse: true}, kind: raise_statement}
match:
  any: [{kind: else_clause}, {kind: elif_clause}]
  inside:
    kind: if_statement
    all:
      - has: {field: consequence, matches: ends}
      - not: {has: {kind: elif_clause, not: {has: {field: consequence, matches: ends}}}}
  regex: ''
message: "Remove the `else`; the `raise` already leaves the branch"
```

## How to fix

1. Delete the `else:` (or turn the `elif` into an `if`) and dedent its body one level.

## Examples

### Python

#### Don't

```python
def check(x):
    if x < 0:
        raise ValueError(x)
    else:
        return x
```

#### Do

```python
def check(x):
    if x < 0:
        raise ValueError(x)
    return x
```
