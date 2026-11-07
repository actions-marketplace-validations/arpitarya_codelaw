---
id: py/missing-trailing-period
title: "A docstring's summary ends with a period"
languages: [python]
fix: suggest
fix_reason: "a period is added to the docstring's text"
stability: preview
ported_from: [ruff/D400]
since: 0.5.0
---

The summary line of a docstring — its first line with text — ends with a period.

**Why:** The summary is a sentence; tools that join it into other text read it as one.

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
  regex: '^[rRuU]?(?:"""|'''''')\s*\S[^\n]*?[^.\s][ \t]*(\n|(?:"""|''''''))'
message: "End the summary with a period"
fix:
  - edit-text: {pattern: '^([rRuU]?(?:"""|'''''')\s*\S[^\n]*?[^.\s])([ \t]*(?:\n|(?:"""|'''''')))', replace: '$1.$2'}
```

## Examples

### Python

#### Don't

```python
def get(user_id):
    """Return the user with this id"""
```

#### Do

```python
def get(user_id):
    """Return the user with this id."""
```
