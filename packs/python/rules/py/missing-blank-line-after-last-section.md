---
id: py/missing-blank-line-after-last-section
title: "A blank line follows the last docstring section"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D413]
since: 0.5.0
---

The last section of a docstring is followed by a blank line before the closing quotes. The fix adds it. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** The closing quotes are not part of the last section; the blank line says so.

```codelaw
native: py/missing-blank-line-after-last-section
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
