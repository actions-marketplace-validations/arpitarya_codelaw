---
id: py/unnecessary-ellipsis
title: "No ... beside other statements"
languages: [python]
fix: auto
stability: preview
ported_from: [pylint/unnecessary-ellipsis]
since: 0.5.0
---

A `...` statement in a block that has other statements (a docstring counts) is removed.

**Why:** `...` is only a placeholder for an empty block.

```codelaw
match:
  kind: expression_statement
  regex: '^\.\.\.$'
  inside: {kind: block}
  any:
    - follows: {stopBy: end, not: {kind: comment}, regex: '\S'}
    - precedes: {stopBy: end, not: {kind: comment}, regex: '\S'}
message: "This `...` is not needed"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
def f():
    """Not built yet."""
    ...
```

#### Do

```python
def f():
    """Not built yet."""
```
