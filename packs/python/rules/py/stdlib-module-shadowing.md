---
id: py/stdlib-module-shadowing
title: "A module does not take a standard-library module's name"
languages: [python]
fix: manual
fix_reason: "renaming the file renames the module, and every import of it"
fix_exception: authored
stability: preview
options:
  strict_checking: { type: "boolean", default: false, description: "also report a module inside a package (`app/logging.py`), not only a top-level one" }
  allowed_modules: { type: "string[]", default: [], description: "standard-library names a module may take" }
ported_from: [ruff/A005]
since: 0.5.0
---

A top-level module — a file at the project root or in `src/`, or a package's `__init__.py` named by its folder — named like a standard-library module (`logging.py`, `json/__init__.py`) is reported, from the pack's table of standard-library module names. A private name (`_thread`) is allowed, and with `strict_checking` off (the default, as Ruff) a module inside a package is not read, since it is imported by its dotted name.

**Why:** A top-level module named `logging` is what `import logging` finds first, here and in every library the program imports.

```codelaw
native: py/stdlib-module-shadowing
```

## How to fix

1. Rename the module to a name the standard library does not use, and update its imports.

## Examples

### Python

#### Don't

```python path=logging.py
def setup():
    pass
```

#### Do

```python path=log_setup.py
def setup():
    pass
```
