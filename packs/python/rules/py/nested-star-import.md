---
id: py/nested-star-import
title: "`import *` only at module level"
languages: [python]
fix: auto
fix_reason: "auto when the module is the project's and the file has one star import: the star becomes the names the module binds that the function uses, read from the module; a suggestion with two. A module outside the project offers no fix (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [ruff/F406]
since: 0.1.0
---

`from module import *` appears only at the top level of a module.

**Why:** Inside a function or class a star import is a `SyntaxError` in Python 3.

```codelaw
native: py/nested-star-import
```

## How to fix

1. `codelaw fix` replaces the star with the names the function uses, where the module is the project's.
2. For another module, replace the star with the names the function uses.

## Examples

### Python

#### Don't

```py
# shapes.py
def area(r):
    return 3.14 * r * r


def perimeter(r):
    return 6.28 * r
# app.py
def measure():
    from shapes import *
    return area(2)
```

#### Do

```py
# shapes.py
def area(r):
    return 3.14 * r * r


def perimeter(r):
    return 6.28 * r
# app.py
def measure():
    from shapes import area
    return area(2)
```
