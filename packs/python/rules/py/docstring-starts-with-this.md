---
id: py/docstring-starts-with-this
title: "A docstring does not start with \"This\""
languages: [python]
fix: manual
fix_reason: "rewording the summary is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/D404]
since: 0.5.0
---

A docstring's first word is not `This`: write `"""Return the user."""`, not `"""This function returns the user."""`.

**Why:** "This function" restates what the reader already knows; the summary says what it does.

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
  regex: '^[rRuU]?(?:"""|'''''')\s*This\b'
message: "Start the docstring with what it does, not \"This\""
```

## How to fix

1. Reword the summary to start with its verb: `This function returns the user.` → `Return the user.`

## Examples

### Python

#### Don't

```python
def get(user_id):
    """This function returns the user."""
```

#### Do

```python
def get(user_id):
    """Return the user."""
```
