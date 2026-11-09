---
id: py/missing-type-doc
title: "Every documented parameter has a type"
languages: [python]
fix: manual
fix_reason: "the type is the author's to state"
fix_exception: authored
stability: preview
options: { accept_no_param_doc: { type: "boolean", default: true, description: "do not ask for parameter documentation in a docstring that documents no parameter at all" }, check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/missing-type-doc]
since: 0.5.0
---

A function whose docstring documents some of its parameters gives each one a type: in the docstring (`:param int x:`, `:type x: int`, `x (int):`, `x : int`) or as an annotation in the signature. `self`, `cls`, starred parameters and names starting with `_`, `ignored_` or `unused_` are not asked for one. A docstring that documents no parameter is left alone unless `accept_no_param_doc` is `false`. Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A parameter documented without a type leaves the reader to find it in the body.

```codelaw
native: py/missing-type-doc
```

## How to fix

1. Write the type beside the parameter's entry, or annotate the parameter in the signature.

## Examples

### Python

#### Don't

```python
def area(width, height):
    """Return the area.

    :param int width: The width.
    :param height: The height.
    """
    return width * height
```

#### Do

```python
def area(width, height):
    """Return the area.

    :param int width: The width.
    :param int height: The height.
    """
    return width * height
```
