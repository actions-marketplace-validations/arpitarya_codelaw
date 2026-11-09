---
id: py/useless-type-doc
title: "A parameter that is ignored by name has no documented type"
languages: [python]
fix: suggest
fix_reason: "the type can be what the author wants kept; dropping it is offered, not applied"
stability: preview
options: { check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/useless-type-doc]
since: 0.5.0
---

A parameter whose name starts with `_`, `ignored_` or `unused_` is one the function does not use, and a type written for it in the docstring (`:type _x: int`, `:param int _x:`, `_x (int):`, `_x : int`) is noise. The suggestion drops the type. An annotation in the signature is not read. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A type for a parameter the function ignores says it matters.

```codelaw
native: py/useless-type-doc
```

## Examples

### Python

#### Don't

```python
def area(width, _unused):
    """Return the area.

    Args:
        width: The width.
        _unused (int): Not used.
    """
    return width
```

#### Do

```python
def area(width, _unused):
    """Return the area.

    Args:
        width: The width.
        _unused: Not used.
    """
    return width
```
