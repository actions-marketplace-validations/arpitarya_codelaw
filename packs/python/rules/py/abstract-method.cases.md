# py/abstract-method — cases

### case: a method left abstract

```py
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...


class Square(Shape):
    pass
```

```expect
9:7 py/abstract-method
```

### case: the method is implemented

```py
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...


class Square(Shape):
    def area(self):
        return 1
```

```expect
(none)
```

### case: an abstract subclass

```py
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...


class Round(Shape, ABC):
    pass
```

```expect
(none)
```

### case: two methods, one implemented

```py
from abc import ABC, abstractmethod


class Shape(ABC):
    @abstractmethod
    def area(self): ...

    @abstractmethod
    def perimeter(self): ...


class Square(Shape):
    def area(self):
        return 1
```

```expect
12:7 py/abstract-method
```
