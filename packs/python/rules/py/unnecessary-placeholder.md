---
id: py/unnecessary-placeholder
title: "No pass beside other statements"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PIE790, pylint/unnecessary-pass]
since: 0.5.0
---

A `pass` in a block that has other statements (a docstring counts) is removed.

**Why:** `pass` is only needed where a block would otherwise be empty.

```codelaw
match:
  kind: pass_statement
  inside: {kind: block}
  any:
    - follows: {stopBy: end, not: {kind: comment}, regex: '\S'}
    - precedes: {stopBy: end, not: {kind: comment}, regex: '\S'}
message: "This `pass` is not needed"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
def f():
    """Do nothing yet."""
    pass
```

#### Do

```python
def f():
    """Do nothing yet."""
```
