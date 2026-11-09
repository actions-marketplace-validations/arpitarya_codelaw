---
id: py/multiple-constructor-doc
title: "A constructor's parameters are documented once"
languages: [python]
fix: manual
fix_reason: "which of the two documents the parameters is the author's call"
fix_exception: authored
stability: preview
ported_from: [pylint/multiple-constructor-doc]
since: 0.5.0
---

A class docstring and the docstring of its `__init__` or `__new__` do not both document the constructor's parameters. A function named `__init__` outside a class is not a constructor.

**Why:** Two descriptions of the same parameters drift apart; one is the one the reader trusts.

```codelaw
native: py/multiple-constructor-doc
```

## How to fix

1. Keep the parameters in one of the two docstrings and delete them from the other.

## Examples

### Python

#### Don't

```python
class Box:
    """A box.

    Args:
        size: The size.
    """

    def __init__(self, size):
        """Make a box.

        Args:
            size: The size.
        """
        self.size = size
```

#### Do

```python
class Box:
    """A box."""

    def __init__(self, size):
        """Make a box.

        Args:
            size: The size.
        """
        self.size = size
```
