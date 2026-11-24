---
id: py/import-private-name
title: "Do not import another package's private names"
languages: [python]
scope: project
fix: manual
fix_reason: "which public name or module the code should use instead, if there is one, only the author can say"
fix_exception: value
stability: preview
ported_from: [ruff/PLC2701]
since: 0.5.2
---

A `from m import _name`, or an import whose module path has a part that starts with an underscore (`from pkg._impl import f`, `import pkg._impl`), is reported when the importing file belongs to another top-level package than the one imported from. A package may use its own private names: the first part of the importing module's dotted name (read from the project's roots, as `py/import-error` reads them) is compared with the first part of the import. Not reported: a relative import (it is inside the package), a `__dunder__` name or module part, an import under `if TYPE_CHECKING:` or used only in annotations, and a side-effect import. Standard-library and installed packages are judged the same way, since the rule reads only the import, not the module's file: `from os import _exit` is reported. Ruff's `PLC2701` (preview), behaviour read from its documentation and rewritten.

**Why:** A leading underscore says the name is the package's own to change; code that imports it breaks on a release the maintainers called compatible.

```codelaw
graph:
  python:
    imports: private
message: "Private name import `$NAME` from external module `$SOURCE`"
```

## How to fix

1. Import the public name the package offers for the same thing.
2. If there is none, ask the package for one, or copy the few lines you need.

## Examples

### Python

#### Don't

```py files
# pkg/__init__.py
# pkg/_impl.py
def _helper(): ...
# other/__init__.py
# other/app.py
from pkg import _helper

_helper()
```

#### Do

```py files
# pkg/__init__.py
# pkg/_impl.py
def helper(): ...
# other/__init__.py
# other/app.py
from pkg import helper

helper()
```
