# py/abstract-class-instantiated — cases

### case: an abstract class

```py
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...


shape = Shape()
```

```expect
9:9 py/abstract-class-instantiated
```

### case: a concrete subclass

```py
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...


class Square(Shape):
    def area(self):
        return 1


square = Square()
```

```expect
(none)
```

### case: an abstract subclass instantiated

```py
from abc import ABC, abstractmethod


class A(ABC):
    @abstractmethod
    def f(self): ...


class B(A):
    pass


b = B()
```

```expect
13:5 py/abstract-class-instantiated
```

### case: abstractmethod without ABC

```py
from abc import abstractmethod


class A:
    @abstractmethod
    def f(self): ...


a = A()
```

```expect
(none)
```
