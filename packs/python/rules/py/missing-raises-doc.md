---
id: py/missing-raises-doc
title: "A docstring that documents exceptions documents the ones the function raises"
languages: [python]
fix: manual
fix_reason: "when and why the exception is raised is the author's to write"
fix_exception: authored
stability: preview
options: { accept_no_raise_doc: { type: "boolean", default: true, description: "do not ask for exception documentation in a docstring that documents no exception at all" }, check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/missing-raises-doc]
since: 0.5.0
---

A function whose docstring has exception documentation (`:raises X:`, a `Raises:` section, a NumPy `Raises` section) documents every exception its own `raise` statements raise: `raise X`, `raise X(...)`, and a bare `raise` or `raise exc` in an `except` clause (the exceptions that clause handles). An exception caught by a `try` around the `raise`, `NotImplementedError`, and anything the checker cannot tell the class of (a package it cannot see) are left out. A documented class covers the classes derived from it, in the file or among the built-in exceptions, and a name is compared by its last part (`calendar.IllegalMonthError` and `IllegalMonthError`). A property setter without a docstring is read against its getter's. With the default `accept_no_raise_doc` a docstring that documents no exception is left alone; set it to `false` to have every raised exception documented. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A caller reads the docstring to learn what to catch; an exception left out is a surprise.

```codelaw
native: py/missing-raises-doc
```

## How to fix

1. Add each exception named in the finding to the docstring's exception documentation, with when it is raised.

## Examples

### Python

#### Don't

```python
def divide(a, b):
    """Divide a by b.

    Raises:
        TypeError: If an argument is not a number.
    """
    if b == 0:
        raise ZeroDivisionError("b is zero")
    return a / b
```

#### Do

```python
def divide(a, b):
    """Divide a by b.

    Raises:
        TypeError: If an argument is not a number.
        ZeroDivisionError: If b is zero.
    """
    if b == 0:
        raise ZeroDivisionError("b is zero")
    return a / b
```
