---
id: py/deprecated-import
title: "Import a name from where it lives now"
languages: [python]
fix: suggest
fix_reason: "the import moves to the module that has the name today; an import with a comment inside it, or one that shares its line with another statement and has other names, is reported without an edit, and a name that was also renamed (`typing.List` is `list`) has no import to write"
finding_target: { python: { from: "3.0" } }
stability: preview
ported_from: [ruff/UP035]
since: 0.5.0
---

A `from … import …` takes a name from a module it has left: `collections` for the abstract base classes (now `collections.abc`), `typing` for names moved to `collections.abc`, `collections` and `re` (from Python 3.9, `Callable` from 3.10), `typing_extensions` for names the standard library has since taken in, `typing.io`, `typing.re`, `pipes`, `mypy_extensions` and `backports.strenum`. Each name is judged against the project's target, so `Self` from `typing_extensions` is reported from 3.11 and not before. The names come from a table taken from Ruff 0.16.9's, in `data/python/moved-names.json`. Names that moved and were renamed (`typing.List` is now `list`, `typing.AbstractSet` is `collections.abc.Set`) are reported without an edit. A star import and a relative import are left alone, and names Ruff keeps importing from `typing_extensions` for its backported fixes are not in the table. With no target the rule does nothing.

**Why:** The old location may be removed, and a name from `typing_extensions` that the standard library has takes a dependency for nothing.

```codelaw
native: py/deprecated-import
```

## How to fix

1. Import the name from the module the message gives.

## Examples

### Python

#### Don't

```python python=3.9
from collections import Mapping
```

#### Do

```python python=3.9
from collections.abc import Mapping
```
