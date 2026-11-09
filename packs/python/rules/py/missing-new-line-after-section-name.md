---
id: py/missing-new-line-after-section-name
title: "A NumPy section name ends its line"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D406]
since: 0.5.0
---

In a NumPy-style docstring a section name stands alone on its line, followed by its underline: `Returns`, not `Returns:`. The fix removes what follows the name. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** NumPy style marks a section by the underline; a colon after the name is Google style, and mixing them confuses the parsers.

```codelaw
native: py/missing-new-line-after-section-name
```

## Examples

### Python

#### Don't

```python
def f(a):
    """Return a.

    Returns:
    -------
    int
        The value.

    """
```

#### Do

```python
def f(a):
    """Return a.

    Returns
    -------
    int
        The value.

    """
```
