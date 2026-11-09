---
id: py/redundant-returns-doc
title: "A docstring documents a returned value only if there is one"
languages: [python]
fix: suggest
fix_reason: "the docstring can be stale or the function can be missing its return; deleting the documentation is offered, not applied"
stability: preview
ported_from: [pylint/redundant-returns-doc]
since: 0.5.0
---

A docstring that documents a return value (`:returns:`, `:rtype:`, a `Returns:` section) belongs to a function with a `return` of a value; a function that returns nothing or only `None` is reported. A generator is reported too when the docstring is Google or NumPy style (those have `Yields`); a Sphinx docstring uses `:returns:` for what a generator yields. Abstract methods, overloads and stubs are not read.

**Why:** Documentation of a value that is not returned misleads the caller.

```codelaw
native: py/redundant-returns-doc
```

## Examples

### Python

#### Don't

```python
def log(message):
    """Log a message.

    Returns:
        None: Nothing.
    """
    print(message)
```

#### Do

```python
def log(message):
    """Log a message.
    """
    print(message)
```
