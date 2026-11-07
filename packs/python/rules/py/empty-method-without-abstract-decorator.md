---
id: py/empty-method-without-abstract-decorator
title: "Empty methods in an abstract base class are abstract"
languages: [python]
fix: suggest
fix_reason: "`@abstractmethod` goes right above the `def`, imported from `abc` when it isn't: a subclass that doesn't define the method can no longer be instantiated, so review the optional hooks. A name `abstractmethod` taken by something else is manual"
fix_exception: value
stability: preview
ported_from: [ruff/B027]
since: 0.1.0
---

In a class that inherits from `ABC`, a method whose body is empty is marked `@abstractmethod`.

**Why:** An empty, unmarked method lets subclasses forget to implement it.

```codelaw
native: py/empty-method-without-abstract-decorator
```

## How to fix

1. Add `@abstractmethod` if subclasses must implement it.
2. Otherwise give it a docstring saying it is an optional hook.

## Examples

### Python

#### Don't

```py
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...

    def draw(self):
        pass
```

#### Fixed

```py
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...

    @abstractmethod
    def draw(self):
        pass
```

#### Do

```py
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...

    @abstractmethod
    def draw(self): ...
```
