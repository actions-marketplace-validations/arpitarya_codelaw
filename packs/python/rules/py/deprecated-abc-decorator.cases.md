# py/deprecated-abc-decorator — cases

### case: a name imported from abc

```py
from abc import ABC, abstractclassmethod


class Shape(ABC):
    @abstractclassmethod
    def make(cls): ...
```

```expect
5:5 py/deprecated-abc-decorator
```

```fixed
from abc import ABC, abstractclassmethod, abstractmethod


class Shape(ABC):
    @classmethod
    @abstractmethod
    def make(cls): ...
```

### case: abstractmethod itself, and a name from elsewhere, are fine

```py
import abc
from mylib import abstractproperty


class Shape(abc.ABC):
    @abc.abstractmethod
    def area(self): ...

    @abstractproperty
    def size(self): ...
```

```expect
(none)
```
