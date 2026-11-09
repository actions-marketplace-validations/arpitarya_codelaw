---
id: py/docstring-missing-yields
title: "A yielded value is documented"
languages: [python]
fix: manual
fix_reason: "what the values mean is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/DOC402]
since: 0.5.0
---

A generator that yields values other than `None` has a `Yields` section in its docstring.

**Why:** Callers read the docstring to learn what a generator produces.

```codelaw
native: py/docstring-missing-yields
```

## How to fix

1. Add a `Yields` section describing the values.

## Examples

### Python

#### Don't

```python
def count(n):
    """Yield the numbers below n.

    Args:
        n: The bound.

    """
    for i in range(n):
        yield i
```

#### Do

```python
def count(n):
    """Yield the numbers below n.

    Args:
        n: The bound.

    Yields:
        The next number.

    """
    for i in range(n):
        yield i
```
