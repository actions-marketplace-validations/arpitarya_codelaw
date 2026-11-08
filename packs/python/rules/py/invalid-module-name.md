---
id: py/invalid-module-name
title: "A module's name is lowercase with underscores"
languages: [python]
fix: manual
fix_reason: "renaming the file renames the module, and every import of it in and outside the project"
fix_exception: authored
stability: preview
options:
  ignore_names: { type: "string[]", default: [], description: "module names that are not reported; `*` and `?` are wildcards" }
ported_from: [ruff/N999]
since: 0.5.0
---

A module inside a package — a file whose folder has an `__init__.py`, or a package's own `__init__.py`, named by its folder — is named in lowercase letters, digits and underscores, not starting with a digit (PEP 8): `order_lines.py`, not `OrderLines.py` or `order-lines.py`. In a `migrations/` or `versions/` folder a name may start with a digit (`0001_initial.py`). A file outside any package is a script and is not read.

**Why:** A module name is written in every import; a dash cannot be imported at all, and a capital breaks on a case-insensitive file system.

```codelaw
native: py/invalid-module-name
```

## How to fix

1. Rename the file (or folder) in lowercase with underscores, and update its imports.

## Examples

### Python

#### Don't

```python path=Billing/__init__.py
TAX = 0.2
```

#### Do

```python path=billing/__init__.py
TAX = 0.2
```
