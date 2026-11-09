---
id: py/overindented-section
title: "A docstring section header is not over-indented"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D214]
since: 0.5.0
---

The header of a docstring section (`Args:`, `Returns`, `Parameters` …) is indented like the docstring, not deeper. The fix removes the extra indentation from the header line. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** A header pushed to the right of the docstring's text is read as part of the previous section's content.

```codelaw
native: py/overindented-section
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
