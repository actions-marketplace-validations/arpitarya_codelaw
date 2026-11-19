---
id: py/deprecated-class
title: "No standard-library class the documentation deprecates"
languages: [python]
fix: manual
fix_reason: "the replacement is another class the documentation names, which the table does not hold, and its uses change with it"
fix_exception: value
finding_target: { python: { from: "3.0" } }
stability: preview
ported_from: [pylint/deprecated-class]
since: 0.5.0
---

A class that CPython's documentation deprecates, named in a `from module import Class` or as `module.Class`, reported when the project's target is at or past the version that deprecates it (`importlib.abc.ResourceLoader`, `asyncio.AbstractEventLoopPolicy`, `typing.Text` and the like). The name is read as the file's imports resolve it; a class bound to another name by assignment is not followed.

**The table** is the data package `@codelaw/data-python-deprecations` / `codelaw-data-python-deprecations` (generated from CPython's library reference, the Python Software Foundation License; L6 decision 15), read only when this rule is on. Without the package the rule reports nothing. It lists only names the documentation deprecates as a whole, by the Python version that deprecates them (and the one that removes them): a deprecation of one way of calling a function is not in it.

**The target** is the project's oldest supported Python (`[targets] python`, else `requires-python`; SR-CONFIG decision 11f). A name is reported once the target has reached the version that deprecates it, as Pylint's `py-version` does; with no target the rule does nothing.

**Why:** A deprecated class warns when used and is removed in a later Python.

```codelaw
native: py/deprecated-class
```

## How to fix

1. Read the documentation of the class: its deprecation note names the replacement.
2. Replace the class and the code that subclasses or calls it.

## Examples

### Python

#### Don't

```python python=3.9
from importlib.abc import ResourceLoader


class Loader(ResourceLoader):
    pass
```

#### Do

```python python=3.9
from importlib.resources.abc import TraversableResources


class Loader(TraversableResources):
    pass
```

## Differences from the source

Pylint's table of deprecated classes is its own and larger; codelaw's is the documentation's. codelaw reports the `from module import Class` and every `module.Class` written out, and does not follow a class that reached a variable. The PEP 585 aliases (`typing.Dict`) are `py/deprecated-typing-alias`'s, not this rule's.
