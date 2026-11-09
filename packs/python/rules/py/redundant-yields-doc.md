---
id: py/redundant-yields-doc
title: "A docstring documents a yielded value only if there is one"
languages: [python]
fix: suggest
fix_reason: "the docstring can be stale or the function can be missing its yield; deleting the documentation is offered, not applied"
stability: preview
ported_from: [pylint/redundant-yields-doc]
since: 0.5.0
---

A Google or NumPy docstring with a `Yields` section belongs to a function that has a `yield` (or `yield from`); a function that does not is reported. Sphinx docstrings have no yields field of their own and are not read.

**Why:** Documentation of a yield that is not there misleads the caller.

```codelaw
native: py/redundant-yields-doc
```

## Examples

### Python

#### Don't

```python
def total(values):
    """Add up the values.

    Yields:
        int: The sum.
    """
    return sum(values)
```

#### Do

```python
def total(values):
    """Add up the values.
    """
    return sum(values)
```
