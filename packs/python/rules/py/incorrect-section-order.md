---
id: py/incorrect-section-order
title: "Docstring sections come in the conventional order"
languages: [python]
fix: manual
fix_reason: "moving a section moves its content, and the reading order is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/D420]
since: 0.5.0
---

The sections of a docstring follow the convention's order: for Google style `Args`, `Returns`, `Yields`, `Raises`; for NumPy style summary, `Parameters`, `Attributes`, `Methods`, `Returns`, `Yields`, `Receives`, `Other Parameters`, `Raises`, `Warns`, `Warnings`, `See Also`, `Notes`, `References`, `Examples`. A section out of order is reported against the highest-ranked section before it. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** A fixed order lets a reader (and a tool) find a section without scanning the docstring.

```codelaw
native: py/incorrect-section-order
```

## How to fix

1. Move the section named in the message above the one it names.

## Examples

### Python

#### Don't

```python
def f(a):
    """Return a.

    Returns:
        The value.

    Args:
        a: The value.

    """
```

#### Do

```python
def f(a):
    """Return a.

    Args:
        a: The value.

    Returns:
        The value.

    """
```
