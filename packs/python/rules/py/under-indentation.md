---
id: py/under-indentation
title: "A docstring's lines are not indented less than its quotes"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D207]
since: 0.5.0
---

A line inside a multi-line docstring starts at least as far right as the line the docstring begins on. The fix indents the line to that column.

**Why:** Documentation tools strip the common indentation of a docstring's lines; one line pulled to the left changes what they strip from the rest.

```codelaw
native: py/under-indentation
```

## Examples

### Python

#### Don't

```python
def f():
    """Return one.

  Longer text.
    """
```

#### Do

```python
def f():
    """Return one.

    Longer text.
    """
```
