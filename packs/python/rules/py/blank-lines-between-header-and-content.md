---
id: py/blank-lines-between-header-and-content
title: "No blank lines between a docstring section's header and its content"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D412]
since: 0.5.0
---

The content of a docstring section starts on the line after its header (or its underline). The fix removes the blank lines, except a single one before a reStructuredText directive (`.. code-block::`), which needs it. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** A blank line after the header ends the section as far as the section parsers go.

```codelaw
native: py/blank-lines-between-header-and-content
```

## Examples

### Python

#### Don't

```python
def f(a):
    """Return a.

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

    """
```
