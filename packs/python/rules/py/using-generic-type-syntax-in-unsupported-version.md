---
id: py/using-generic-type-syntax-in-unsupported-version
title: "No PEP 695 syntax below Python 3.12"
languages: [python]
fix: manual
fix_reason: "type parameters have an older spelling with `TypeVar` and `Generic`, but writing it is the author's work"
fix_exception: restructure
finding_target: { python: { below: "3.12" } }
stability: preview
ported_from: [pylint/using-generic-type-syntax-in-unsupported-version]
since: 0.5.0
---

A `class` or a `def` with type parameters (`class Box[T]:`, `def first[T](items)`) and a `type X = …` statement are reported when the project's target is below Python 3.12, the first version with PEP 695 syntax. With no target the rule does nothing.

**Why:** The file is a `SyntaxError` on the older versions the project says it supports.

```codelaw
match:
  any:
    - { kind: type_alias_statement }
    - { kind: class_definition, has: { kind: type_parameter, field: type_parameters } }
    - { kind: function_definition, has: { kind: type_parameter, field: type_parameters } }
message: "PEP 695 syntax needs Python 3.12; the target is older"
```

## How to fix

1. Raise the project's target to 3.12, or declare the type variables with `TypeVar` and `Generic`.

## Examples

### Python

#### Don't

```python python=3.11
class Box[T]:
    def __init__(self, item: T) -> None:
        self.item = item
```

#### Do

```python python=3.11
from typing import Generic, TypeVar

T = TypeVar("T")


class Box(Generic[T]):
    def __init__(self, item: T) -> None:
        self.item = item
```
