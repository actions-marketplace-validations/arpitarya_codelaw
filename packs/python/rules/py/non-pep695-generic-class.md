---
id: py/non-pep695-generic-class
title: "A generic class uses type parameters, not `Generic`"
languages: [python]
fix: auto
fix_reason: "auto from Python 3.12, where the syntax exists; below the target, or with none set, the finding is a suggestion; a `Generic` that is not the last base, a name that is not a `TypeVar` the file assigns, and a comment in the bases are reported without an edit"
fix_target: { python: "3.12" }
stability: preview
ported_from: [ruff/UP046]
since: 0.5.0
---

A class defined at module level whose last positional base is `Generic[T, …]` (or `typing.Generic`) declares its variables as type parameters: `class Box[T]:`. Each name in `Generic[…]` must be a `TypeVar`, `TypeVarTuple` or `ParamSpec` the file assigns with a string name; `bound=` and constraints are written as `T: int` and `T: (int, str)`, `TypeVarTuple` as `*Ts`, `ParamSpec` as `**P`, and `typing.AnyStr` as `AnyStr: (bytes, str)`. A `TypeVar` with a `default` is not rewritten. The assignment of the variable stays, so another function may still use it. The finding is reported for every target; the syntax is Python 3.12's, and the edit is automatic only when the project's target is.

**Why:** Type parameters declare the variable where it is used and need no `TypeVar` and no `Generic` base.

```codelaw
native: py/non-pep695-generic-class
```

## Examples

### Python

#### Don't

```python
from typing import Generic, TypeVar

T = TypeVar("T")


class Box(Generic[T]):
    def __init__(self, item: T) -> None:
        self.item = item
```

#### Do

```python
from typing import Generic, TypeVar

T = TypeVar("T")


class Box[T]:
    def __init__(self, item: T) -> None:
        self.item = item
```
