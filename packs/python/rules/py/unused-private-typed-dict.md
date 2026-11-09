---
id: py/unused-private-typed-dict
title: "A private TypedDict is used"
languages: [python]
fix: suggest
fix_reason: "deleting the declaration can break an importer of the private name in another file"
stability: preview
ported_from: [ruff/PYI049]
since: 0.5.0
---

A class derived from `TypedDict`, or a `TypedDict(...)` call assigned to a name, that starts with one underscore is used in the file. The edit deletes it.

**Why:** A private typed dict nothing refers to is dead code.

```codelaw
native: py/unused-private-typed-dict
```

## Examples

### Python

#### Don't

```python
from typing import TypedDict


class _Options(TypedDict):
    verbose: bool
```

#### Do

```python
from typing import TypedDict


class _Options(TypedDict):
    verbose: bool


def run(options: _Options) -> None: ...
```

#### Fixed

```python
from typing import TypedDict
```
