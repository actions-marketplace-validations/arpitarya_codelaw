---
id: py/docstring-extraneous-exception
title: "A documented exception is raised"
languages: [python]
fix: manual
fix_reason: "whether the entry or the code is out of date is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/DOC502]
since: 0.5.0
---

Each exception named in a `Raises` section of a docstring is one the function body raises. Abstract methods are not checked.

**Why:** An entry for an exception that is never raised misleads the caller into handling it.

```codelaw
native: py/docstring-extraneous-exception
```

## How to fix

1. Remove the entry, or raise the exception it describes.

## Examples

### Python

#### Don't

```python
def divide(a, b):
    """Divide a by b.

    Raises:
        ValueError: If b is zero.

    """
    return a / b
```

#### Do

```python
def divide(a, b):
    """Divide a by b."""
    return a / b
```
