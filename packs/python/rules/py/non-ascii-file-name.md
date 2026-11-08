---
id: py/non-ascii-file-name
title: "A module's file name is ASCII"
languages: [python]
fix: manual
fix_reason: "renaming the file renames the module, and every import of it"
fix_exception: authored
stability: preview
ported_from: [pylint/non-ascii-file-name]
since: 0.5.0
---

A Python file whose module name — its file name without `.py`, or its folder's for an `__init__.py` — holds a character outside ASCII is reported.

**Why:** An import of a non-ASCII module depends on the file system's Unicode normalisation, which differs between macOS, Windows and Linux.

```codelaw
native: py/non-ascii-file-name
```

## How to fix

1. Rename the file with ASCII characters, and update its imports.

## Examples

### Python

#### Don't

```python path=café.py
MENU = []
```

#### Do

```python path=cafe.py
MENU = []
```
