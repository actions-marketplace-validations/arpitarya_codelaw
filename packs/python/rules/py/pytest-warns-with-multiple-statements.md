---
id: py/pytest-warns-with-multiple-statements
title: "pytest.warns wraps one simple statement"
languages: [python]
fix: manual
fix_reason: "which statement is meant to warn, and where the others go, is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/PT031]
since: 0.5.0
---

The body of `with pytest.warns(...):` is one simple statement — not several, and not an `if`, `for`, `while`, `with` or `try`.

**Why:** An earlier statement that emits the same warning makes the test pass without reaching the line it is about.

```codelaw
match:
  kind: with_statement
  all:
    - has:
        kind: with_clause
        has: {kind: with_item, has: {kind: call, has: {field: function, regex: '^pytest\.warns$'}}}
    - has:
        field: body
        any:
          - has: {nthChild: 2, not: {kind: comment}}
          - has: {nthChild: 1, any: [{kind: if_statement}, {kind: for_statement}, {kind: while_statement}, {kind: with_statement}, {kind: try_statement}]}
message: "Keep only the statement that should warn inside `pytest.warns`"
```

## How to fix

1. Move the setup statements above the `with` block, so only the call that should warn is inside it.

## Examples

### Python

#### Don't

```python
with pytest.warns(DeprecationWarning):
    client = make()
    client.old_api()
```

#### Do

```python
client = make()
with pytest.warns(DeprecationWarning):
    client.old_api()
```
