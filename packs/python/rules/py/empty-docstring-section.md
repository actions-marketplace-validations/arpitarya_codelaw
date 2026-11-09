---
id: py/empty-docstring-section
title: "A docstring section has content"
languages: [python]
fix: manual
fix_reason: "what the section says is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/D414]
since: 0.5.0
---

A section of a docstring (`Args:`, `Returns`, …) is followed by something other than blank lines. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** A header with nothing under it documents nothing.

```codelaw
native: py/empty-docstring-section
```

## How to fix

1. Write the section's content, or delete the header.

## Examples

### Python

#### Don't

```python
def f():
    """Do it.

    Returns:

    """
```

#### Do

```python
def f():
    """Do it.

    Returns:
        Nothing.

    """
```
