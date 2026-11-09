---
id: py/unused-private-type-var
title: "A private TypeVar is used"
languages: [python]
fix: suggest
fix_reason: "deleting the declaration can break an importer of the private name in another file"
stability: preview
ported_from: [ruff/PYI018]
since: 0.5.0
---

A `TypeVar`, `ParamSpec` or `TypeVarTuple` bound to a name that starts with one underscore is used in the file, in code or in a quoted annotation. The edit deletes the declaration. A name in `__all__` or a string annotation counts as used.

**Why:** A private type variable nothing refers to is left over from a refactor.

```codelaw
native: py/unused-private-type-var
```

## Examples

### Python

#### Don't

```python
from typing import TypeVar

_T = TypeVar("_T")
```

#### Do

```python
from typing import TypeVar

_T = TypeVar("_T")


def first(items: list[_T]) -> _T:
    return items[0]
```

#### Fixed

```python
from typing import TypeVar
```
