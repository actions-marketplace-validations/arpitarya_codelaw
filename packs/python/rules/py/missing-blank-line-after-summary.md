---
id: py/missing-blank-line-after-summary
title: "A docstring's summary is followed by a blank line"
languages: [python]
fix: suggest
fix_reason: "a blank line is added to the docstring's text"
stability: preview
ported_from: [ruff/D205]
since: 0.5.0
---

In a docstring of more than one line whose summary is on the first line, the second line is blank.

**Why:** Tools that show only the summary take everything up to the first blank line.

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
  regex: '^[rRuU]?(?:"""|'''''')[ \t]*\S[^\n]*\n[ \t]*\S'
  not: {regex: '^[rRuU]?(?:"""|'''''')[ \t]*\S[^\n]*\n[ \t]*(?:"""|'''''')$'}
message: "Put a blank line after the summary"
fix:
  - edit-text: {pattern: '^([^\n]*\n)', replace: "$1\n"}
```

## Examples

### Python

#### Don't

```python
def get(user_id):
    """Return the user with this id.
    Raises KeyError when there is none.
    """
```

#### Do

```python
def get(user_id):
    """Return the user with this id.

    Raises KeyError when there is none.
    """
```
