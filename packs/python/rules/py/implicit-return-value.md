---
id: py/implicit-return-value
title: "return None, not return, in a function that returns values"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RET502]
since: 0.5.0
---

In a function that returns values elsewhere, a bare `return` is written `return None`.

**Why:** The explicit `None` shows the function's result on every path.

```codelaw
match:
  kind: return_statement
  regex: '^return$'
  inside:
    kind: function_definition
    stopBy: {kind: function_definition}
    has: {stopBy: {kind: function_definition}, kind: return_statement, regex: '^return\s+\S', not: {regex: '^return\s+None$'}}
message: "Write `return None`"
fix: "return None"
```

## Examples

### Python

#### Don't

```python
def find(xs, k):
    if not xs:
        return
    return xs.index(k)
```

#### Do

```python
def find(xs, k):
    if not xs:
        return None
    return xs.index(k)
```
