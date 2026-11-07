---
id: py/first-word-uncapitalized
title: "A docstring's first word is capitalised"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D403]
since: 0.5.0
---

A docstring that starts with a word of lowercase letters starts it with a capital: `"""Return the user."""`. A word with other characters in it (`iOS`, `foo_bar`, `x2`) is left alone.

**Why:** The summary is a sentence.

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
  kind: string_content
  pattern: $C
  regex: '^[a-z]+([\s.,:;!?]|$)'
  inside: {matches: docstring}
message: "Capitalise the first word"
fix: "$C|upper-first"
```

## Examples

### Python

#### Don't

```python
def get(user_id):
    """return the user with this id."""
```

#### Do

```python
def get(user_id):
    """Return the user with this id."""
```
