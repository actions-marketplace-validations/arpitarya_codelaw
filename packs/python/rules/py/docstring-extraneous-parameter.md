---
id: py/docstring-extraneous-parameter
title: "A docstring documents only parameters the function has"
languages: [python]
fix: manual
fix_reason: "whether the entry is stale or the parameter was dropped by mistake is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/DOC102]
since: 0.5.0
---

A function with no `*args` or `**kwargs` has an `Args` or `Parameters` section that names only parameters of the signature.

**Why:** An entry for a parameter that no longer exists is documentation that was not updated with the code.

```codelaw
native: py/docstring-extraneous-parameter
```

## How to fix

1. Remove the entry, or restore the parameter it describes.

## Examples

### Python

#### Don't

```python
def area(width):
    """Return the area.

    Args:
        width: The width.
        height: The height.

    """
    return width
```

#### Do

```python
def area(width):
    """Return the area.

    Args:
        width: The width.

    """
    return width
```
