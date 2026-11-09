---
id: py/unused-private-type-alias
title: "A private type alias is used"
languages: [python]
fix: suggest
fix_reason: "deleting the declaration can break an importer of the private name in another file"
stability: preview
ported_from: [ruff/PYI047]
since: 0.5.0
---

A `TypeAlias`-annotated assignment or a `type` statement whose name starts with one underscore is used in the file. The edit deletes the alias.

**Why:** A private alias nothing refers to is dead code.

```codelaw
native: py/unused-private-type-alias
```

## Examples

### Python

#### Don't

```python
from typing import TypeAlias

_Number: TypeAlias = int | float
```

#### Do

```python
from typing import TypeAlias

_Number: TypeAlias = int | float


def double(x: _Number) -> _Number:
    return x * 2
```

#### Fixed

```python
from typing import TypeAlias
```
