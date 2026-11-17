---
id: py/future-required-type-annotation
title: "An annotation newer than the target needs the `__future__` import"
languages: [python]
fix: auto
finding_target: { python: { from: "3.7", below: "3.10" } }
stability: preview
ported_from: [ruff/FA102]
since: 0.5.0
---

A parameter, a return type or a variable outside a function is annotated with a PEP 604 union (`int | None`, from Python 3.10) or a PEP 585 collection (`list[int]`, from 3.9) that the project's target does not run, and the file lacks `from __future__ import annotations`. Reported only for a target from 3.7 up to, not including, 3.10; the union is judged against 3.10 and the collection against 3.9, so a 3.9 target hears only about unions. With no target the rule does nothing. The fix adds the import after the module docstring and any other `__future__` imports.

**Why:** Without the import the interpreter evaluates the annotation when the function is defined, and `int | None` or `list[int]` then raises a `TypeError` on the older version.

```codelaw
native: py/future-required-type-annotation
```

## How to fix

1. Add `from __future__ import annotations` after the module docstring.

## Examples

### Python

#### Don't

```python python=3.9
import math


def clamp(value: float, limit: int | None) -> float:
    return math.floor(value)
```

#### Do

```python python=3.9
from __future__ import annotations

import math


def clamp(value: float, limit: int | None) -> float:
    return math.floor(value)
```
