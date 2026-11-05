---
id: py/invalid-envvar-value
title: "os.getenv takes a string name"
languages: [python]
fix: manual
fix_reason: "the variable's name is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE1507]
since: 0.5.0
---

The name passed to `os.getenv` is a string.

**Why:** Any other type raises TypeError.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^os\.getenv$}
  - has:
      field: arguments
      has:
        nthChild: 1
        any:
        - {kind: integer}
        - {kind: float}
        - {kind: none}
        - {kind: 'true'}
        - {kind: 'false'}
        - {kind: list}
        - {kind: dictionary}
message: The variable name must be a string
```

## How to fix

1. Pass the variable's name as a string.

## Examples

### Python

#### Don't

```python
port = os.getenv(8080)
```

#### Do

```python
port = os.getenv("PORT")
```
