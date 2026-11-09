---
id: py/docstring-extraneous-returns
title: "A `Returns` section is for a function that returns"
languages: [python]
fix: manual
fix_reason: "whether the section or the code is out of date is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/DOC202]
since: 0.5.0
---

A docstring has a `Returns` section only when the function has a `return` with a value. Abstract methods are not checked.

**Why:** A section describing a value that is never returned misleads the caller.

```codelaw
native: py/docstring-extraneous-returns
```

## How to fix

1. Remove the `Returns` section, or return the value it describes.

## Examples

### Python

#### Don't

```python
def log(message):
    """Write a message.

    Args:
        message: The text.

    Returns:
        Nothing.

    """
    print(message)
```

#### Do

```python
def log(message):
    """Write a message.

    Args:
        message: The text.

    """
    print(message)
```
