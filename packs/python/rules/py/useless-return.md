---
id: py/useless-return
title: "No return None at the end"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLR1711]
since: 0.5.0
---

A bare `return` or `return None` as the last statement of a function is removed.

**Why:** Falling off the end returns `None` already.

```codelaw
match:
  kind: return_statement
  regex: '^return(\s+None)?$'
  nthChild: {position: 1, reverse: true}
  follows: {regex: '.'}
  inside: {kind: block, inside: {kind: function_definition}}
message: "This return does nothing"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
def save(x):
    store(x)
    return None
```

#### Do

```python
def save(x):
    store(x)
```
