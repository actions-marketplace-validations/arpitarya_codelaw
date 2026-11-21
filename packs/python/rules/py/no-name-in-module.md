---
id: py/no-name-in-module
title: "A name imported from the project's own module must be defined there"
languages: [python]
scope: project
fix: manual
fix_reason: "which name was meant, or whether the module should define it, only the author can say"
fix_exception: value
stability: preview
ported_from: [pylint/no-name-in-module]
since: 0.5.0
---

`from m import x` is reported when `m` is a module of the project, `m` binds no `x` at its top level (a `def`, `class`, assignment, import, loop or `with` target, or a name `from … import *` brings in) and `m` has no submodule `x`. A module whose names cannot all be known is not judged: one that defines `__getattr__`, changes its `__path__`, reads `globals()` or `vars()`, or has a `from … import *` of a module outside the project or with a dynamic `__all__`. A guarded import (`try … except ImportError`, `if TYPE_CHECKING:`) and an import of a module outside the project (the standard library, an installed package) are not judged. A name a build step adds to the module (a compiled extension, a generated file) is reported until the file is in the checkout.

**Why:** the import raises `ImportError: cannot import name` when it runs.

```codelaw
graph:
  python:
    imports: unexported
message: "No name `$NAME` in module `$SOURCE`"
```

## How to fix

1. Import the name the module defines, or fix the spelling.
2. Or define the name in the module.

## Examples

### Python

#### Don't

```py files
# shop/__init__.py
# shop/prices.py
def total(items):
    return sum(items)
# shop/app.py
from shop.prices import totall

print(totall([1]))
```

#### Do

```py files
# shop/__init__.py
# shop/prices.py
def total(items):
    return sum(items)
# shop/app.py
from shop.prices import total

print(total([1]))
```
