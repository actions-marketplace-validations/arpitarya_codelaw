---
id: py/redefined-slots-in-subclass
title: "A subclass's `__slots__` does not repeat its base's"
languages: [python]
fix: manual
fix_reason: "which class keeps the slot is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLW0244, pylint/redefined-slots-in-subclass]
since: 0.5.0
---

A class defined in the file whose `__slots__` lists a name that a base class defined in the same file already lists (directly or through its own bases). Only literal string names are read.

**Why:** A repeated slot adds a second descriptor and wastes memory in every instance.

```codelaw
native: py/redefined-slots-in-subclass
```

## How to fix

1. Remove the name from the subclass's `__slots__`.

## Examples

### Python

#### Don't

```python
class Base:
    __slots__ = ("name",)


class Child(Base):
    __slots__ = ("name", "age")
```

#### Do

```python
class Base:
    __slots__ = ("name",)


class Child(Base):
    __slots__ = ("age",)
```
