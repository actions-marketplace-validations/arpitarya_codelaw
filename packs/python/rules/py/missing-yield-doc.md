---
id: py/missing-yield-doc
title: "A generator documents what it yields"
languages: [python]
fix: manual
fix_reason: "what the generator yields is the author's to describe"
fix_exception: authored
stability: preview
options: { accept_no_yields_doc: { type: "boolean", default: true, description: "do not ask for yield documentation in a docstring that documents no yielded value" }, check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/missing-yield-doc]
since: 0.5.0
---

A function with a `yield` (or `yield from`) has a description of the yielded values in its docstring: a `Yields:` section with text (Google), a `Yields` section with an indented description (NumPy); a Sphinx docstring uses `:returns:`. This is for a project that wants every generator described: Pylint's default `accept_no_yields_doc` accepts one that is not, so the rule does nothing until the option is `false`. A function without a docstring is asked too. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A caller reads the docstring to learn what the generator produces.

```codelaw
native: py/missing-yield-doc
```

## How to fix

1. Describe the yielded values in the docstring's own style.

## Examples

### Python

#### Don't

```python options={"accept_no_yields_doc":false}
def count(limit):
    """Count up to limit."""
    for i in range(limit):
        yield i
```

#### Do

```python options={"accept_no_yields_doc":false}
def count(limit):
    """Count up to limit.

    Yields:
        The next number.
    """
    for i in range(limit):
        yield i
```
