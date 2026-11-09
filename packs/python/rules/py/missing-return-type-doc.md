---
id: py/missing-return-type-doc
title: "A function that returns a value documents its type"
languages: [python]
fix: manual
fix_reason: "the type is the author's to state"
fix_exception: authored
stability: preview
options: { accept_no_return_doc: { type: "boolean", default: true, description: "do not ask for return documentation in a docstring that documents no return value" }, check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/missing-return-type-doc]
since: 0.5.0
---

A function with a `return` statement that returns a value, and no return annotation, has the type of it in its docstring: `:rtype:` (Sphinx), `type: description` in a `Returns:` section (Google), a type line in a `Returns` section (NumPy). A property may carry it in its first line (`int: the value`). This is for a project that wants every returned type written down: Pylint's default `accept_no_return_doc` accepts a function that does not, so the rule does nothing until the option is `false`. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A caller reads the docstring to learn what type comes back.

```codelaw
native: py/missing-return-type-doc
```

## How to fix

1. Write the type in the docstring, or annotate the return type of the function.

## Examples

### Python

#### Don't

```python options={"accept_no_return_doc":false}
def area(width, height):
    """Return the area.

    Returns:
        The area.
    """
    return width * height
```

#### Do

```python options={"accept_no_return_doc":false}
def area(width, height):
    """Return the area.

    Returns:
        float: The area.
    """
    return width * height
```
