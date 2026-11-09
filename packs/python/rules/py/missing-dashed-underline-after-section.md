---
id: py/missing-dashed-underline-after-section
title: "A NumPy section name is underlined with dashes"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D407]
since: 0.5.0
---

In a NumPy-style docstring each section name has a line of dashes under it. The fix adds the line, or turns an underline of `=` into dashes, as long as the name. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** The underline is what marks a NumPy section; without it the header is a line of text.

```codelaw
native: py/missing-dashed-underline-after-section
```

## Examples

### Python

#### Don't

```python
def f(a):
    """Return a.

    Parameters
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
