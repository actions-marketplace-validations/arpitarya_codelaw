---
id: py/over-indentation
title: "A docstring's lines are not indented more than its quotes"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D208]
since: 0.5.0
---

Every line of a multi-line docstring after the first is indented deeper than the line the docstring starts on, or the line holding the closing quotes is. The fix removes the extra indentation; the relative indentation of the lines is kept.

**Why:** A docstring pushed to the right of its quotes looks like an indented block (a code example, a quotation) when it is not.

```codelaw
native: py/over-indentation
```

## Examples

### Python

#### Don't

```python
def f():
    """Return one.

        Longer text.
        More.
    """
```

#### Do

```python
def f():
    """Return one.

    Longer text.
    More.
    """
```
