---
id: py/star-import-usage
title: "No names that may only come from a star import"
languages: [python]
fix: auto
fix_reason: "auto when exactly one star import of the file binds the name and the module is the project's: that star becomes the names the file uses from it, read from the module; a suggestion when the file has two star imports. A name no project module binds offers no fix (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [ruff/F405]
since: 0.1.0
---

In a module with `from module import *`, every name the module reads is defined locally or imported by name.

**Why:** With a star import in play, a misspelled name and a name the star import provides look the same; neither can be checked.

```codelaw
native: py/star-import-usage
```

## How to fix

1. `codelaw fix` replaces the star that binds the name with the names the file uses from it, where the module is the project's.
2. Otherwise find which star-imported module provides the name, and import it by name from that module.

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
