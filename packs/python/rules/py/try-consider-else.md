---
id: py/try-consider-else
title: "A return after the guarded work goes in else"
languages: [python]
fix: manual
fix_reason: "the statements move from the `try` into an `else` block"
fix_exception: restructure
stability: preview
ported_from: [ruff/TRY300]
since: 0.5.0
---

A `try` whose body ends in `return`, after other statements, and that has an `except` handler and no `else`, moves the `return` into an `else` block.

**Why:** Only the statements that can raise belong in the `try`; the `return` in it widens what the handler catches.

```codelaw
match:
  kind: return_statement
  nthChild: {position: 1, reverse: true}
  inside:
    kind: block
    has: {nthChild: 2, not: {kind: comment}}
    inside:
      kind: try_statement
      field: body
      has: {kind: except_clause}
      not: {has: {kind: else_clause}}
message: "Move the `return` into an `else` block"
```

## How to fix

1. Add an `else:` after the handlers and move the `return` (and the statements after the last one that can raise) into it.

## Examples

### Python

#### Don't

```python
def load(p):
    try:
        data = read(p)
        return parse(data)
    except OSError:
        return None
```

#### Do

```python
def load(p):
    try:
        data = read(p)
    except OSError:
        return None
    else:
        return parse(data)
```
