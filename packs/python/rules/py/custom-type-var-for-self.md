---
id: py/custom-type-var-for-self
title: "A method is annotated with `Self`, not a custom `TypeVar`"
languages: [python]
fix: auto
fix_reason: "auto from Python 3.11, where `typing.Self` exists; below the target, or with none set, the finding is a suggestion; a method whose type variable also appears in a string annotation is reported without an edit"
fix_target: { python: "3.11" }
stability: preview
ported_from: [ruff/PYI019]
since: 0.5.0
---

An instance method annotates `self` with a `TypeVar` the file assigns, or a class method (`@classmethod`, `__new__`, `__init_subclass__`, `__class_getitem__`) annotates `cls` as `type[T]`. The edit drops the annotation, writes `Self` wherever the method uses the variable, and imports `Self` from `typing`, joining an existing `from typing import …`. Static methods, abstract methods, overloads and methods of a metaclass are not read, and a variable declared as a PEP 695 type parameter is not read either.

**Why:** `Self` says what the `TypeVar` says in one word, and needs no variable to declare.

```codelaw
native: py/custom-type-var-for-self
```

## Examples

### Python

#### Don't

```python
from typing import TypeVar

_S = TypeVar("_S", bound="Shape")


class Shape:
    def scale(self: _S, factor: float) -> _S:
        return self
```

#### Do

```python
from typing import TypeVar, Self

_S = TypeVar("_S", bound="Shape")


class Shape:
    def scale(self, factor: float) -> Self:
        return self
```
