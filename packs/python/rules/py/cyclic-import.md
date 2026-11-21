---
id: py/cyclic-import
title: "A module must not import its way back to itself"
languages: [python]
scope: project
fix: manual
fix_reason: "breaking a cycle moves code into a third module or inverts a dependency, which is a change of structure across files"
fix_exception: restructure
stability: preview
ported_from: [pylint/cyclic-import]
since: 0.5.0
---

Each `import` and `from … import` of a module of the project whose target imports the importing module again — through any number of modules — is reported at the statement, with the way back (`Cyclic import (pkg.a -> pkg.b -> pkg.a)`). `from pkg import b` is an import of `pkg` and, when `pkg/b.py` is a module, of `pkg.b`, as Pylint reads it. An import under `if TYPE_CHECKING:` is not run, so it closes no cycle. Pylint reports one message per cycle; this rule reports each import that closes one, so a fix is found at the line to change. `importlib.import_module` and a `sys.path` change are not followed.

**Why:** a module in a cycle sees its partner half-initialised, so a name that works in one import order raises `ImportError` or `AttributeError` in another.

```codelaw
graph:
  cycle:
    form: [import]
message: "Cyclic import"
```

## How to fix

1. Move what both modules need into a third module that imports neither.
2. Or import inside the function that needs it, or under `if TYPE_CHECKING:` when only a type is needed.

## Examples

### Python

#### Don't

```py files
# pkg/__init__.py
# pkg/a.py
from pkg import b


def run():
    return b.helper()
# pkg/b.py
from pkg.a import run


def helper():
    return run
```

#### Do

```py files
# pkg/__init__.py
# pkg/shared.py
def helper():
    return 1
# pkg/a.py
from pkg import shared


def run():
    return shared.helper()
# pkg/b.py
from pkg.a import run


def again():
    return run
```
