---
id: py/missing-return-doc
title: "A function that returns a value documents it"
languages: [python]
fix: manual
fix_reason: "what the function returns is the author's to describe"
fix_exception: authored
stability: preview
options: { accept_no_return_doc: { type: "boolean", default: true, description: "do not ask for return documentation in a docstring that documents no return value" }, check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/missing-return-doc]
since: 0.5.0
---

A function with a `return` statement that returns a value (not a bare `return` or `return None`) has a description of it in its docstring: `:returns:` (Sphinx), a `Returns:` section with text (Google), a `Returns` section with an indented description (NumPy). A property may carry the description in its first line. This is for a project that wants every returned value described: Pylint's default `accept_no_return_doc` accepts a function that does not, so the rule does nothing until the option is `false`. A function without a docstring is asked too. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A caller reads the docstring to learn what comes back.

```codelaw
native: py/missing-return-doc
```

## How to fix

1. Describe the returned value in the docstring's own style.

## Examples

### Python

#### Don't

```python options={"accept_no_return_doc":false}
def area(width, height):
    """Return the area."""
    return width * height
```

#### Do

```python options={"accept_no_return_doc":false}
def area(width, height):
    """Return the area.

    Returns:
        The area.
    """
    return width * height
```
