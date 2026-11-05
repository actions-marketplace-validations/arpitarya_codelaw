---
id: py/unnecessary-return-none
title: "return, not return None, when nothing else is returned"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RET501]
since: 0.5.0
---

In a function that never returns a value, `return None` is written `return`.

**Why:** The explicit `None` suggests the function returns values.

```codelaw
match:
  kind: return_statement
  regex: ^return\s+None$
  inside:
    kind: function_definition
    stopBy: {kind: function_definition}
    all:
    - not:
        has:
          stopBy: {kind: function_definition}
          kind: return_statement
          regex: ^return\s+\S
          not: {regex: ^return\s+None$}
    - not:
        has: {field: return_type, kind: type}
message: Write a bare `return`
fix: return
```

## Examples

### Python

#### Don't

```python
def save(x):
    if not x:
        return None
    store(x)
```

#### Do

```python
def save(x):
    if not x:
        return
    store(x)
```
