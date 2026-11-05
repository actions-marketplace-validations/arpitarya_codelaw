---
id: py/if-with-same-arms
title: "Merge if branches with the same body"
languages: [python]
fix: manual
fix_reason: "the conditions are joined with `or` and one branch deleted, which restructures the chain"
fix_exception: restructure
stability: preview
ported_from: [ruff/SIM114]
since: 0.5.0
---

An `if` and the `elif` right after it with identical one-statement bodies are merged into one `if a or b:`.

**Why:** Two branches that do the same thing are one branch.

```codelaw
match:
  kind: elif_clause
  has: {field: consequence, pattern: $BODY}
  any:
  - follows:
      kind: elif_clause
      has: {field: consequence, pattern: $BODY}
  - inside:
      kind: if_statement
      all:
      - has: {field: consequence, pattern: $BODY}
      - has: {kind: elif_clause, nthChild: 3, pattern: $$MATCH}
message: This branch does the same as the one before; merge the conditions with `or`
```

## How to fix

1. Join the two conditions with `or` and delete the duplicate branch.

## Examples

### Python

#### Don't

```python
if a:
    go()
elif b:
    go()
```

#### Do

```python
if a or b:
    go()
```
