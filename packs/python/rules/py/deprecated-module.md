---
id: py/deprecated-module
title: "No standard-library module the documentation deprecates"
languages: [python]
fix: manual
fix_reason: "the replacement is another module or package the documentation names, which the table does not hold, and the calls move with it"
fix_exception: value
finding_target: { python: { from: "3.0" } }
stability: preview
ported_from: [pylint/deprecated-module]
since: 0.5.0
---

An `import` or `from … import` of a standard-library module that CPython's documentation deprecates (`asyncore`, `distutils`, `imp`, `cgi`, `telnetlib` and the other PEP 594 modules), reported when the project's target is at or past the version that deprecates it. A submodule is read as its module: `from distutils.core import setup` is `distutils`'s.

**The table** is the data package `@codelaw/data-python-deprecations` / `codelaw-data-python-deprecations` (generated from CPython's library reference, the Python Software Foundation License; L6 decision 15), read only when this rule is on. Without the package the rule reports nothing. It lists only names the documentation deprecates as a whole, by the Python version that deprecates them (and the one that removes them): a deprecation of one way of calling a function is not in it.

**The target** is the project's oldest supported Python (`[targets] python`, else `requires-python`; SR-CONFIG decision 11f). A name is reported once the target has reached the version that deprecates it, as Pylint's `py-version` does; with no target the rule does nothing.

**Why:** A deprecated module warns when imported and is removed in a later Python: on the target versions that have already removed it, the import is an `ImportError`.

```codelaw
native: py/deprecated-module
```

## How to fix

1. Read the documentation page of the module: each deprecation names what replaces it (`asyncio` for `asyncore`, `importlib` for `imp`, `setuptools` for `distutils`).
2. Move the code to the replacement, or raise the project's target past the removal only if the module's own package is vendored.

## Examples

### Python

#### Don't

```python python=3.8
import asyncore


class Handler(asyncore.dispatcher):
    pass
```

#### Do

```python python=3.8
import asyncio


async def handle(reader, writer):
    pass
```

## Differences from the source

Pylint's rule also knows modules the user lists in its `deprecated-modules` option; codelaw reads only the documentation's table, and the project's target takes the place of Pylint's `py-version`. A module the documentation no longer marks deprecated is not reported.
