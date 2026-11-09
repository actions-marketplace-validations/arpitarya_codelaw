---
id: py/docstring-missing-returns
title: "A returned value is documented"
languages: [python]
fix: manual
fix_reason: "what the value means is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/DOC201]
since: 0.5.0
---

A function that returns a value other than `None` has a `Returns` section in its docstring. Properties, `__new__`, stubs, and functions annotated `-> None` are not checked.

**Why:** Callers read the docstring to learn what comes back.

```codelaw
native: py/docstring-missing-returns
```

## How to fix

1. Add a `Returns` section describing the value.

## Examples

### Python

#### Don't

```python
def area(width, height):
    """Return the area of a rectangle.

    Args:
        width: The width.
        height: The height.

    """
    return width * height
```

#### Do

```python
def area(width, height):
    """Return the area of a rectangle.

    Args:
        width: The width.
        height: The height.

    Returns:
        The area.

    """
    return width * height
```
