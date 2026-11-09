---
id: py/differing-type-doc
title: "A docstring types only the parameters that exist"
languages: [python]
fix: manual
fix_reason: "whether the type line is misspelled or stale is the author's"
fix_exception: value
stability: preview
options: { check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/differing-type-doc]
since: 0.5.0
---

A name given a type in the docstring (`:type x: int`, `:param int x:`, `x (int):`, `x : int`) is a parameter of the function. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A type line for a parameter that is not there is a misspelling, or documentation that was not updated with the code.

```codelaw
native: py/differing-type-doc
```

## How to fix

1. Correct the name in the type entry to the parameter it describes, or delete the entry.

## Examples

### Python

#### Don't

```python
def area(width):
    """Return the area.

    :param width: The width.
    :type widht: int
    """
    return width
```

#### Do

```python
def area(width):
    """Return the area.

    :param width: The width.
    :type width: int
    """
    return width
```
