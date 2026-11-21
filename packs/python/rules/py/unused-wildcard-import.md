---
id: py/unused-wildcard-import
title: "A wildcard import's names must be used"
languages: [python]
scope: project
fix: manual
fix_reason: "replacing the wildcard with the names the file reads is a choice of which to import, and a name read only by `eval` or a test must stay"
fix_exception: value
stability: preview
ported_from: [pylint/unused-wildcard-import]
since: 0.5.0
---

A `from m import *` of a module of the project is reported with the names it brings in that the file never reads. The names are `m`'s `__all__` when it is a literal, else every top-level name of `m` that does not start with `_` (imports included, and the names of a `from … import *` it makes itself). A name counts as read when the file reads it, lists it in its own `__all__`, or has it as a word inside a string (a quoted annotation, an `eval`), which is conservative. A module outside the project, one with a dynamic `__all__`, and an `__init__.py` (a re-export on purpose, as Pylint's `init-import` default) are not judged. One finding per statement, listing up to ten names.

**Why:** a wildcard import hides where each name comes from, and one that brings in names nothing uses is dead weight that can also shadow a name the file means to define.

```codelaw
graph:
  python:
    imports: wildcard_unused
message: "Unused names from wildcard import of `$SOURCE`"
```

## How to fix

1. Replace the wildcard with an import of the names the file reads: `from m import used_one, used_two`.

## Examples

### Python

#### Don't

```py files
# shop/__init__.py
# shop/consts.py
TAX = 0.2
FEE = 1
# shop/app.py
from shop.consts import *

print(TAX)
```

#### Do

```py files
# shop/__init__.py
# shop/consts.py
TAX = 0.2
FEE = 1
# shop/app.py
from shop.consts import TAX

print(TAX)
```
