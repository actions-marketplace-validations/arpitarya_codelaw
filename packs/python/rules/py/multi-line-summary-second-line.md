---
id: py/multi-line-summary-second-line
title: "A multi-line docstring's summary starts on the second line"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D213]
since: 0.5.0
---

A docstring of more than one line, whose closing quotes are on their own line, starts its summary on the line after the opening quotes. It is the opposite convention to `py/multi-line-summary-first-line`; a project turns on one of the two.

**Why:** One place for the summary keeps docstrings consistent; `inspect.getdoc` reads both the same.

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
  regex: '^[rRuU]?(?:"""|'''''')[ \t]*\S[^\n]*\n[\s\S]*\n[ \t]*(?:"""|'''''')$'
message: "Start the summary on the second line"
fix:
  - edit-text: {pattern: '^([rRuU]?(?:"""|''''''))[ \t]*(\S[\s\S]*\n([ \t]*)(?:"""|''''''))$', replace: "$1\n$3$2"}
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
    """
    Return the user with this id.

    Raises KeyError when there is none.
    """
```
