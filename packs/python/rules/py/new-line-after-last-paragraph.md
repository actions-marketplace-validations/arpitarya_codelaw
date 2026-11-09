---
id: py/new-line-after-last-paragraph
title: "The closing quotes of a multi-line docstring are on their own line"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D209]
since: 0.5.0
---

A triple-quoted docstring with more than one line of text puts its closing quotes on a line of their own. The fix moves them down, at the docstring's indentation.

**Why:** Closing quotes at the end of the last line are easy to miss when the docstring is edited or extended.

```codelaw
native: py/new-line-after-last-paragraph
```

## Examples

### Python

#### Don't

```python
def f():
    """Return one.

    Longer text."""
```

#### Do

```python
def f():
    """Return one.

    Longer text.
    """
```
