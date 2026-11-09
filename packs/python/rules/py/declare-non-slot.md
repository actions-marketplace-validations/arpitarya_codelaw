---
id: py/declare-non-slot
title: "An annotation in a class with `__slots__` names a slot"
languages: [python]
fix: manual
fix_reason: "whether to add the name to `__slots__` or drop the annotation is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/declare-non-slot]
since: 0.5.0
---

A class with a non-empty literal `__slots__` annotates a name without a value (`c: bool`) that no `__slots__` of the class or its ancestors lists, `__dict__` is not among the slots, and the annotation is not a `ClassVar`. A name with a value, an empty `__slots__`, a base without `__slots__` or one the file imports are not read.

**Why:** The annotation promises an instance attribute the layout has no room for, so the first assignment raises `AttributeError`.

```codelaw
native: py/declare-non-slot
```

## How to fix

1. Add the name to `__slots__`, or
2. mark it `ClassVar[...]` when it is a class attribute.

## Examples

### Python

#### Don't

```python
class Point:
    __slots__ = ("x", "y")

    x: int
    y: int
    label: str
```

#### Do

```python
class Point:
    __slots__ = ("x", "y", "label")

    x: int
    y: int
    label: str
```
