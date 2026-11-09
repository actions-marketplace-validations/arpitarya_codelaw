---
id: py/abstract-method
title: "A concrete class implements the abstract methods it inherits"
languages: [python]
fix: manual
fix_reason: "what the method should do is the author's to write"
fix_exception: authored
stability: preview
ported_from: [pylint/abstract-method]
since: 0.5.0
---

A class that is not itself abstract (it neither lists `ABC` or `Protocol` as a base nor sets `metaclass=ABCMeta` nor declares an `@abstractmethod`) inherits an `@abstractmethod` from a class of this file and does not override it. One finding for each such method. Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** The class cannot be instantiated, and a reader cannot tell from it which methods are missing.

```codelaw
native: py/abstract-method
```

## How to fix

1. Implement the method in the class.

## Examples

### Python

#### Don't

```python
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...


class Square(Shape):
    pass
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
```
