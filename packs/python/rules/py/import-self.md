---
id: py/import-self
title: "A module does not import itself"
languages: [python]
fix: manual
fix_reason: "the import is either dead or a sign the module was split wrongly; which, and what replaces it, is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLW0406, pylint/import-self]
since: 0.5.0
---

A module that imports its own dotted name — `import pkg.mod` or `from pkg import mod` in `pkg/mod.py`, `from . import mod` too — is reported. The module's name is read from its path: its folders up to the first without an `__init__.py`. A file outside any package is not read.

**Why:** The import runs the module a second time under another name or hands back the half-built module; either way it is not what was meant.

```codelaw
native: py/import-self
```

## How to fix

1. Remove the import and use the module's own names directly.

## Examples

### Python

#### Don't

```python path=shop/__init__.py
import shop

PRICE = shop.BASE
```

#### Do

```python path=shop/__init__.py
BASE = 1
PRICE = BASE
```
