---
id: py/unnecessary-multiline-docstring
title: "A one-line docstring fits on one line"
languages: [python]
fix: suggest
fix_reason: "the docstring's text loses its surrounding newlines; `inspect.getdoc` reads the same, `__doc__` does not"
stability: preview
ported_from: [ruff/D200]
since: 0.5.0
---

A docstring whose text is one line is written on one line with its quotes: `"""Return the user."""`.

**Why:** Spreading one line over three adds nothing a reader needs.

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
  regex: '^[rRuU]?(?:"""|'''''')[ \t]*\n?[ \t]*[^\s"''][^\n]*?[ \t]*\n[ \t]*(?:"""|'''''')$'
message: "Put the one-line docstring on one line"
fix:
  - edit-text: {pattern: '^([rRuU]?(?:"""|''''''))\s*([^\n]*?)\s*((?:"""|''''''))$', replace: '$1$2$3'}
```

## Examples

### Python

#### Don't

```python
def get(user_id):
    """
    Return the user with this id.
    """
```

#### Do

```python
def get(user_id):
    """Return the user with this id."""
```
