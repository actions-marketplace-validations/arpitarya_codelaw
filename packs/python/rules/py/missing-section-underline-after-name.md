---
id: py/missing-section-underline-after-name
title: "A section underline follows its name directly"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D408]
since: 0.5.0
---

The line of dashes under a NumPy-style section name comes on the next line, not after blank lines. The fix removes the blank lines. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** Parsers look for the underline on the line right under the name.

```codelaw
native: py/missing-section-underline-after-name
```

## Examples

### Python

#### Don't

```python
def f(a):
    """Return a.

    Parameters

    ----------
    a : int
        The value.

    """
```

#### Do

```python
def f(a):
    """Return a.

    Parameters
    ----------
    a : int
        The value.

    """
```
