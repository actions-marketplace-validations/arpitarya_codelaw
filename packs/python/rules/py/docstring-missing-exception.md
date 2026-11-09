---
id: py/docstring-missing-exception
title: "A raised exception is documented"
languages: [python]
fix: manual
fix_reason: "when the exception is raised is the author's to say"
fix_exception: authored
stability: preview
ported_from: [ruff/DOC501]
since: 0.5.0
---

An exception a function raises (`raise Name`, `raise Name(...)`, or a bare `raise` or `raise exc` inside an `except` clause) is named in a `Raises` section of its docstring. `Exception`, `BaseException` and `NotImplementedError` are not required, and nested functions and classes are not read. A name is read as a builtin, an import or a class defined in the file.

**Why:** Callers read the docstring to learn what they must handle.

```codelaw
native: py/docstring-missing-exception
```

## How to fix

1. Add a `Raises` section naming the exception and when it is raised.

## Examples

### Python

#### Don't

```python
def divide(a, b):
    """Divide a by b.

    Args:
        a: The dividend.
        b: The divisor.

    Returns:
        The quotient.

    """
    if b == 0:
        raise ValueError("b is zero")
    return a / b
```

#### Do

```python
def divide(a, b):
    """Divide a by b.

    Args:
        a: The dividend.
        b: The divisor.

    Returns:
        The quotient.

    Raises:
        ValueError: If b is zero.

    """
    if b == 0:
        raise ValueError("b is zero")
    return a / b
```
