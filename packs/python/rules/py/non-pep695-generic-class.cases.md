# py/non-pep695-generic-class — cases

### case: a bound, and another base

```py
from typing import Generic, TypeVar

N = TypeVar("N", bound=float)


class Vec(Base, Generic[N]):
    pass
```

```expect
6:17 py/non-pep695-generic-class
```

```fixed
from typing import Generic, TypeVar

N = TypeVar("N", bound=float)


class Vec[N: float](Base):
    pass
```

### case: a name that is not a TypeVar: reported, no edit

```py
from typing import Generic, TypeVar

T = TypeVar("T")


class Box(Generic[T, Other]):
    pass
```

```expect
6:11 py/non-pep695-generic-class
```

### case: a class inside a function is left alone

```py
from typing import Generic, TypeVar

T = TypeVar("T")


def make():
    class Box(Generic[T]):
        pass

    return Box
```

```expect
(none)
```
