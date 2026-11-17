---
id: py/future-rewritable-type-annotation
title: "The `__future__` import would let an annotation drop `typing`"
languages: [python]
fix: auto
finding_target: { python: { from: "3.7", below: "3.10" } }
stability: preview
ported_from: [ruff/FA100]
since: 0.5.0
---

A parameter, a return type or a variable outside a function is annotated with a `typing` name that the built-in or `collections` type replaces (`typing.List`, `typing.Dict`, `typing.Callable`, …) or that `X | Y` replaces (`typing.Optional`, `typing.Union`), and the file lacks `from __future__ import annotations`. Reported only for a target from 3.7 up to, not including, 3.10; the collection names are reported below 3.9 and `Optional` and `Union` below 3.10. With no target the rule does nothing. The fix adds the import after the module docstring and any other `__future__` imports; the annotation is rewritten by `py/non-pep585-annotation`, `py/non-pep604-annotation-optional` and `py/non-pep604-annotation-union`.

**Why:** With the import, annotations are not evaluated, so the shorter built-in forms work on the older version and the `typing` import can go.

```codelaw
native: py/future-rewritable-type-annotation
```

## How to fix

1. Add `from __future__ import annotations` after the module docstring.
2. Write the annotation with the built-in type or `X | Y`.

## Examples

### Python

#### Don't

```python python=3.9
import typing


def first(values: typing.Optional[int]) -> int:
    return values or 0
```

#### Do

```python python=3.9
from __future__ import annotations

import typing


def first(values: typing.Optional[int]) -> int:
    return values or 0
```
