---
id: py/docstring-extraneous-yields
title: "A `Yields` section is for a generator"
languages: [python]
fix: manual
fix_reason: "whether the section or the code is out of date is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/DOC403]
since: 0.5.0
---

A docstring has a `Yields` section only when the function yields. Abstract methods are not checked.

**Why:** A section for values that are never yielded misleads the caller.

```codelaw
native: py/docstring-extraneous-yields
```

## How to fix

1. Remove the `Yields` section, or make the function yield.

## Examples

### Python

#### Don't

```python
def count(n):
    """Return the numbers below n.

    Args:
        n: The bound.

    Yields:
        The next number.

    """
    return list(range(n))
```

#### Do

```python
def count(n):
    """Return the numbers below n.

    Args:
        n: The bound.

    """
    return list(range(n))
```
