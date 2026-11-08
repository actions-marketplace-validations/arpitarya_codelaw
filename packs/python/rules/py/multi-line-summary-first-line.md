---
id: py/multi-line-summary-first-line
title: "A multi-line docstring's summary starts on the first line"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D212, pylint/docstring-first-line-empty]
since: 0.5.0
---

A docstring of more than one line starts its summary right after the opening quotes, not on the next line. It is the opposite convention to `py/multi-line-summary-second-line`; a project turns on one of the two.

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
  regex: '^[rRuU]?(?:"""|'''''')[ \t]*\n[ \t]*\S[^\n]*\n'
message: "Start the summary on the first line"
fix:
  - edit-text: {pattern: '^([rRuU]?(?:"""|''''''))[ \t]*\n[ \t]*', replace: '$1'}
```

## Examples

### Python

#### Don't

```python
def get(user_id):
    """
    Return the user with this id.

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
