---
id: py/missing-yield-type-doc
title: "A generator documents the type it yields"
languages: [python]
fix: manual
fix_reason: "the type is the author's to state"
fix_exception: authored
stability: preview
options: { accept_no_yields_doc: { type: "boolean", default: true, description: "do not ask for yield documentation in a docstring that documents no yielded value" }, check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/missing-yield-type-doc]
since: 0.5.0
---

A function with a `yield` (or `yield from`) and no return annotation has the type of the yielded values in its docstring: `type: description` in a `Yields:` section (Google), a type line in a `Yields` section (NumPy); a Sphinx docstring uses `:rtype:`. This is for a project that wants every generator's type written down: Pylint's default `accept_no_yields_doc` accepts one that is not, so the rule does nothing until the option is `false`. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A caller reads the docstring to learn what type the generator produces.

```codelaw
native: py/missing-yield-type-doc
```

## How to fix

1. Write the type in the docstring, or annotate the return type of the function.

## Examples

### Python

#### Don't

```python options={"accept_no_yields_doc":false}
def count(limit):
    """Count up to limit.

    Yields:
        The next number.
    """
    for i in range(limit):
        yield i
```

#### Do

```python options={"accept_no_yields_doc":false}
def count(limit):
    """Count up to limit.

    Yields:
        int: The next number.
    """
    for i in range(limit):
        yield i
```
