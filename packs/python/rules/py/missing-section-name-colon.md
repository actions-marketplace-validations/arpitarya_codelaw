---
id: py/missing-section-name-colon
title: "A Google section name ends with a colon"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D416]
since: 0.5.0
---

In a Google-style docstring the name of a section is followed by a colon: `Args:`. The fix writes it. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** The colon is what marks a Google-style header; without it the line reads as text.

```codelaw
native: py/missing-section-name-colon
```

## Examples

### Python

#### Don't

```python
def f(a):
    """Return a.

    Args
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
