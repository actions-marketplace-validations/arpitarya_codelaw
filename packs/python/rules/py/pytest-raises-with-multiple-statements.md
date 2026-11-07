---
id: py/pytest-raises-with-multiple-statements
title: "pytest.raises wraps one simple statement"
languages: [python]
fix: manual
fix_reason: "which statement is meant to raise, and where the others go, is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/PT012]
since: 0.5.0
---

The body of `with pytest.raises(...):` is one simple statement — not several, and not an `if`, `for`, `while`, `with` or `try`.

**Why:** An earlier statement that raises the same exception makes the test pass without reaching the line it is about.

```codelaw
match:
  kind: with_statement
  all:
    - has:
        kind: with_clause
        has: {kind: with_item, has: {kind: call, has: {field: function, regex: '^pytest\.raises$'}}}
    - has:
        field: body
        any:
          - has: {nthChild: 2, not: {kind: comment}}
          - has: {nthChild: 1, any: [{kind: if_statement}, {kind: for_statement}, {kind: while_statement}, {kind: with_statement}, {kind: try_statement}]}
message: "Keep only the statement that should raise inside `pytest.raises`"
```

## How to fix

1. Move the setup statements above the `with` block, so only the call that should raise is inside it.

## Examples

### Python

#### Don't

```python
with pytest.raises(KeyError):
    d = load()
    d["missing"]
```

#### Do

```python
d = load()
with pytest.raises(KeyError):
    d["missing"]
```
