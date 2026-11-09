---
id: py/differing-param-doc
title: "A docstring documents only the parameters that exist"
languages: [python]
fix: manual
fix_reason: "whether the entry is misspelled or stale is the author's"
fix_exception: value
stability: preview
options: { check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/differing-param-doc]
since: 0.5.0
---

A name documented as a parameter (`:param x:`, an `Args` entry, a NumPy `Parameters` entry) is a parameter of the function; `self` and `cls` are always fine and a starred parameter may be written with or without its stars. This holds even when a sentence `For the other parameters, see …` excuses the missing entries. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** An entry for a parameter that is not there is a misspelling, or documentation that was not updated with the code.

```codelaw
native: py/differing-param-doc
```

## How to fix

1. Correct the name in the entry to the parameter it describes, or delete the entry.

## Examples

### Python

#### Don't

```python
def area(width):
    """Return the area.

    :param widht: The width.
    """
    return width
```

#### Do

```python
def area(width):
    """Return the area.

    :param width: The width.
    """
    return width
```
