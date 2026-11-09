---
id: py/non-slot-assignment
title: "`__init__` assigns only attributes the class's `__slots__` lists"
languages: [python]
fix: manual
fix_reason: "whether to add the name to `__slots__` or drop the assignment is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0237, pylint/assigning-non-slot]
since: 0.5.0
---

In a class that has no base other than `object` and a `__slots__` of string literals (a list, tuple, set or dict of names), the statements `self.x = …`, `self.x: T = …` and `self.x += …` directly in `__init__` assign only names `__slots__` lists. A name with a `@x.setter` in the class counts as listed. A class with `__dict__` in its slots, or a `__slots__` that is not made of literals, is not read; a class with another base may inherit slots and is not read either.

**Why:** Assigning an attribute that is not in `__slots__` raises `AttributeError` when the object is created.

```codelaw
native: py/non-slot-assignment
```

## How to fix

1. Add the attribute to `__slots__`, or remove the assignment.

## Examples

### Python

#### Don't

```python
class Point:
    __slots__ = ("x",)

    def __init__(self, x, y):
        self.x = x
        self.y = y
```

#### Do

```python
class Point:
    __slots__ = ("x", "y")

    def __init__(self, x, y):
        self.x = x
        self.y = y
```
