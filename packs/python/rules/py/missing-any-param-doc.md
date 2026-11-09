---
id: py/missing-any-param-doc
title: "A function's docstring documents its parameters"
languages: [python]
fix: manual
fix_reason: "what a parameter means is the author's"
fix_exception: authored
stability: preview
options: { accept_no_param_doc: { type: "boolean", default: true, description: "do not ask for parameter documentation in a docstring that documents no parameter at all" }, check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/missing-any-param-doc]
since: 0.5.0
---

A function that has parameters (besides `self` and `cls`) and a docstring that documents none of them, with no annotation to stand for the types, is reported once for all of them. Pylint's default accepts such a docstring, so this rule does nothing until `accept_no_param_doc` is `false`; what is reported when some parameters are documented is `py/missing-param-doc` and `py/missing-type-doc`. A sentence `For the parameters, see …` stands for the documentation. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A docstring that never mentions the parameters sends the reader to the code.

```codelaw
native: py/missing-any-param-doc
```

## How to fix

1. Document each parameter, with its type, in the docstring's own style.

## Examples

### Python

#### Don't

```python options={"accept_no_param_doc":false}
def area(width, height):
    """Return the area."""
    return width * height
```

#### Do

```python options={"accept_no_param_doc":false}
def area(width, height):
    """Return the area.

    :param int width: The width.
    :param int height: The height.
    """
    return width * height
```
