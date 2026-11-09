---
id: py/missing-param-doc
title: "Every parameter of a function is documented when some are"
languages: [python]
fix: manual
fix_reason: "what a parameter means is the author's"
fix_exception: authored
stability: preview
options: { accept_no_param_doc: { type: "boolean", default: true, description: "do not ask for parameter documentation in a docstring that documents no parameter at all" }, check_private_functions: { type: "boolean", default: false, description: "also check functions whose name starts with an underscore (Pylint's `no-docstring-rgx` of `^_` skips them)" } }
ported_from: [pylint/missing-param-doc]
since: 0.5.0
---

A function whose docstring documents some of its parameters documents all of them. The docstring is read as Sphinx (`:param x:`, `:parameter`, `:arg`, `:key`, `:keyword`, with the type written beside the name or in a `:type x:` line), Google (`Args:`, `Keyword Args:`) or NumPy (`Parameters` over a dashed line); `self`, `cls` and names starting with `_`, `ignored_` or `unused_` are not asked for, a `*args` entry may be written `args` or `\*args`, and a sentence `For the other parameters, see …` stands for the rest. With the default `accept_no_param_doc` a docstring that documents no parameter is left alone; set it to `false` to have every function documented (the same option turns on `py/missing-any-param-doc`). A class docstring documents its constructor's parameters (checked on `__init__` and `__new__`). Functions whose name starts with `_` are skipped unless `check_private_functions` is on.

**Why:** A reader of the docstring learns the parameters from it; the one left out is the one they have to read the code for.

```codelaw
native: py/missing-param-doc
```

## How to fix

1. Add an entry for each parameter named in the finding, in the docstring's own style.

## Examples

### Python

#### Don't

```python
def area(width, height):
    """Return the area.

    :param width: The width.
    """
    return width * height
```

#### Do

```python
def area(width, height):
    """Return the area.

    :param width: The width.
    :param height: The height.
    """
    return width * height
```
