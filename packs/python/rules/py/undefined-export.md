---
id: py/undefined-export
title: "`__all__` names only defined names"
languages: [python]
fix: suggest
fix_reason: "the module's names within two edits are choices, the nearest the default, beside removing the entry from `__all__`; `import *` raises AttributeError today. A module with `__getattr__`, or an entry outside a list or tuple literal, is manual"
fix_exception: value
exclude: ["**/__init__.py"]
stability: preview
ported_from: [ruff/F822, pylint/undefined-all-variable]
since: 0.1.0
---

Every name in a module's `__all__` is defined or imported in that module.

**Why:** `from module import *` raises `AttributeError` on a missing name, and readers trust `__all__` as the module's API.

```codelaw
native: py/undefined-export
```

## How to fix

1. Define or import the name, or remove it from `__all__`.

## Examples

### Python

#### Don't

```py
__all__ = ["load", "save"]


def load():
    pass
```

#### Fixed (remove)

```py
__all__ = ["load"]


def load():
    pass
```

#### Do

```py
__all__ = ["load", "save"]


def load():
    pass


def save():
    pass
```

## Differences from the source

`__init__.py` files are excluded, as in Ruff, because their `__all__` may list submodules.
