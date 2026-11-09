---
id: py/no-blank-line-before-section
title: "A blank line comes before each docstring section"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D411]
since: 0.5.0
---

A section header of a docstring has a blank line above it. The fix adds one. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** The blank line separates the section from the paragraph or section above.

```codelaw
native: py/no-blank-line-before-section
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
