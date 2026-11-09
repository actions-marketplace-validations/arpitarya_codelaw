---
id: py/undocumented-param
title: "Every parameter has a description in the docstring"
languages: [python]
fix: manual
fix_reason: "what a parameter means is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/D417]
since: 0.5.0
---

When a docstring has an `Args` (Google) or `Parameters` (NumPy) section, it describes every parameter of the function: not `self` or `cls` of a method, not a name that starts with `_`, and `*args` and `**kwargs` by either spelling (`args` or `*args`). A function whose docstring has no such section is not checked.

**Why:** A parameter list that leaves some out is read as the complete one.

```codelaw
native: py/undocumented-param
```

## How to fix

1. Add an entry for each name the message lists, saying what the argument is.

## Examples

### Python

#### Don't

```python
def add(a, b):
    """Add two numbers.

    Args:
        a: The first number.

    """
```

#### Do

```python
def add(a, b):
    """Add two numbers.

    Args:
        a: The first number.
        b: The second number.

    """
```
