---
id: py/overindented-section-underline
title: "A section underline is not over-indented"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D215]
since: 0.5.0
---

The line of dashes under a NumPy-style section header is indented like the docstring, not deeper. The fix moves it back.

**Why:** An underline further right than its header is not read as the header's underline by the tools that parse the sections.

```codelaw
native: py/overindented-section-underline
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
