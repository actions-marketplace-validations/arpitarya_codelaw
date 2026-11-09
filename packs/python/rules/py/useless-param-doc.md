---
id: py/useless-param-doc
title: "A parameter that is ignored by name is not documented"
languages: [python]
fix: suggest
fix_reason: "an entry can describe something the author wants kept; deleting it is offered, not applied"
stability: preview
options: { check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/useless-param-doc]
since: 0.5.0
---

A parameter whose name starts with `_`, `ignored_` or `unused_` is one the function does not use, and its docstring entry (and any type written for it) is noise. The suggestion deletes the entry's lines. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A function does not describe the parameters it ignores; an entry for one says it matters.

```codelaw
native: py/useless-param-doc
```

## Examples

### Python

#### Don't

```python
def area(width, _unused):
    """Return the area.

    :param width: The width.
    :param _unused: Not used.
    """
    return width
```

#### Do

```python
def area(width, _unused):
    """Return the area.

    :param width: The width.
    """
    return width
```
