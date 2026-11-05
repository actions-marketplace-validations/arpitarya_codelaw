---
id: py/self-assigning-variable
title: "No x = x"
languages: [python]
fix: suggest
fix_reason: "the statement is removed; at module level it may have been meant to raise NameError"
stability: preview
ported_from: [ruff/PLW0127]
since: 0.5.0
---

A name is not assigned to itself.

**Why:** It does nothing; a different name was probably meant.

```codelaw
match:
  kind: expression_statement
  has: {kind: assignment, pattern: $X = $X, not: {has: {field: type, kind: type}}}
constraints:
  X: {kind: identifier}
message: "`$X` is assigned to itself"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
def f(x):
    x = x
    return x
```

#### Do

```python
def f(x):
    return x
```
