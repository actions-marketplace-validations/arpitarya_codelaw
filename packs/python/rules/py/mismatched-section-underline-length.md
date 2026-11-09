---
id: py/mismatched-section-underline-length
title: "A section underline is as long as the section name"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D409]
since: 0.5.0
---

The dashes under a NumPy-style section name are as many as the name's letters. The fix rewrites the underline. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** An underline of another length is a sign the name or the underline was edited alone.

```codelaw
native: py/mismatched-section-underline-length
```

## Examples

### Python

#### Don't

```python
def f(a):
    """Return a.

    Parameters
    -----
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
