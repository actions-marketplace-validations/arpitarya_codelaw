---
id: py/no-blank-line-after-section
title: "A blank line follows each docstring section"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D410]
since: 0.5.0
---

A section of a docstring that is followed by another section ends with a blank line. The fix adds one before the next header. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** The blank line is what ends a section's content; without it the next header reads as one more line of it.

```codelaw
native: py/no-blank-line-after-section
```

## Examples

### Python

#### Don't

```python
def f(a):
    """Return a.

    Args:
        a: The value.
    Returns:
        The value.

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
