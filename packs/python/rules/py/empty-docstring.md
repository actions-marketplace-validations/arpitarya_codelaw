---
id: py/empty-docstring
title: "No empty docstring"
languages: [python]
fix: manual
fix_reason: "what the code does, in words, is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/D419]
since: 0.5.0
---

A docstring is not empty or only whitespace.

**Why:** An empty docstring tells tools there is documentation, and gives none.

```codelaw
utils:
  docstring:
    kind: string
    inside:
      kind: expression_statement
      nthChild: 1
      any:
        - inside: {kind: block, inside: {kind: function_definition}}
        - inside: {kind: block, inside: {kind: class_definition}}
        - inside: {kind: module}
match:
  matches: docstring
  regex: '^[rRuU]?((?:"""|'''''')\s*(?:"""|'''''')|"\s*"|''\s*'')$'
message: "Write the docstring, or remove it"
```

## How to fix

1. Write what the code does, or remove the empty string.

## Examples

### Python

#### Don't

```python
def get(user_id):
    """"""
    return rows[user_id]
```

#### Do

```python
def get(user_id):
    """Return the user with this id."""
    return rows[user_id]
```
