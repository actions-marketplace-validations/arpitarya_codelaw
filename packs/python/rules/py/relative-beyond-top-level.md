---
id: py/relative-beyond-top-level
title: "A relative import must not climb past the top-level package"
languages: [python]
scope: project
fix: manual
fix_reason: "whether to use an absolute import or to move the file into the package depends on how the code is run, which the file does not say"
fix_exception: restructure
stability: preview
ported_from: [pylint/relative-beyond-top-level]
since: 0.5.0
---

A `from .. import x` is reported when it has more dots than the file has package levels: the levels are the folders, from the file's own folder up, that hold an `__init__.py`, so a script outside any package cannot use a relative import at all. A folder without `__init__.py` ends the package, as Pylint reads it; a namespace package above a regular one is not followed. The whole statement is reported, once.

**Why:** Python raises `ImportError: attempted relative import beyond top-level package` (or `with no known parent package`) when the import runs.

```codelaw
graph:
  python:
    imports: beyond_top
message: "Attempted relative import beyond top-level package"
```

## How to fix

1. Use an absolute import (`from top.sibling import x`), or
2. add the missing `__init__.py` so the folder is part of the package.

## Examples

### Python

#### Don't

```py files
# top/__init__.py
# top/leaf.py
from .. import elsewhere
```

#### Do

```py files
# top/__init__.py
# top/elsewhere.py
VALUE = 1
# top/leaf.py
from . import elsewhere
```
