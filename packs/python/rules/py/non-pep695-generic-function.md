---
id: py/non-pep695-generic-function
title: "A generic function uses type parameters"
languages: [python]
fix: auto
fix_reason: "auto from Python 3.12; below the target, or with none set, the finding is a suggestion"
fix_target: { python: "3.12" }
stability: preview
ported_from: [ruff/UP047]
since: 0.5.0
---

A function defined at module level (not in a class or another function) whose parameter annotations name `TypeVar`s the file assigns declares them as type parameters: `def first[T](items: list[T]) -> T:`. The variables are those of the parameters' annotations, in order of first use, written as in `py/non-pep695-generic-class`; a function that already has type parameters, a variable with a `default`, or a name used twice with two meanings is left alone. The finding is reported for every target; the edit is automatic only when the project's target is Python 3.12.

**Why:** Type parameters are scoped to the function and need no module-level `TypeVar`.

```codelaw
native: py/non-pep695-generic-function
```

## Examples

### Python

#### Don't

```python
from typing import TypeVar

T = TypeVar("T")


def first(items: list[T]) -> T:
    return items[0]
```

#### Do

```python
from typing import TypeVar

T = TypeVar("T")


def first[T](items: list[T]) -> T:
    return items[0]
```
