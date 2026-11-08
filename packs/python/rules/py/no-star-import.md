---
id: py/no-star-import
title: "No `from module import *`"
languages: [python]
fix: auto
fix_reason: "auto when the module is the project's and the file has one star import: the star becomes the names the module binds (its `__all__`, else its public top-level names) that the file uses, read from the module; with two star imports a name may come from either, so the fix is a suggestion. A module outside the project offers no fix (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [ruff/F403, pylint/wildcard-import]
since: 0.1.0
---

Modules import the names they use explicitly instead of `from module import *`.

**Why:** A star import hides where names come from, can silently replace local names, and stops a linter from finding undefined names.

```codelaw
native: py/no-star-import
```

## How to fix

1. `codelaw fix` replaces the star with the names the file uses, where the module is the project's.
2. For another module, list the names the file uses from it and replace the star with those names.

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
from shapes import *

print(area(2))
```

#### Do

```py
# shapes.py
def area(r):
    return 3.14 * r * r


def perimeter(r):
    return 6.28 * r
# app.py
from shapes import area

print(area(2))
```

## Differences from the source

Relative star imports (`from .sub import *`) are not reported: that is how a package re-exports its submodules' API, and Ruff leaves them alone in `__init__.py`.
