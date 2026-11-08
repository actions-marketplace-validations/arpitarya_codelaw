---
id: py/abstract-class-instantiated
title: "A class with abstract methods is not instantiated"
languages: [python]
fix: manual
fix_reason: "which concrete subclass to build, or which method to implement, is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/abstract-class-instantiated]
since: 0.5.0
---

A call `Cls(...)` where `Cls` is a class of this file built on `ABC` (or `metaclass=ABCMeta`) that still has an `@abstractmethod` no class on the way down implements. Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** Python raises `TypeError: Can't instantiate abstract class` at the call.

```codelaw
native: py/abstract-class-instantiated
```

## How to fix

1. Instantiate a subclass that implements the abstract methods, or
2. implement them in this class.

## Examples

### Python

#### Don't

```python
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...


shape = Shape()
```

#### Do

```python
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...


class Square(Shape):
    def area(self):
        return 1


shape = Square()
```
