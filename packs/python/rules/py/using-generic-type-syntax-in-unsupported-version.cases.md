# py/using-generic-type-syntax-in-unsupported-version — cases

### case: each form on a 3.11 target

```py python=3.11
class Box[T]:
    pass


def first[T](items: list[T]) -> T:
    return items[0]


type Pair = tuple[int, int]
```

```expect
1:1 py/using-generic-type-syntax-in-unsupported-version
5:1 py/using-generic-type-syntax-in-unsupported-version
9:1 py/using-generic-type-syntax-in-unsupported-version
```

### case: on a 3.12 target

```py python=3.12
class Box[T]:
    pass


type Pair = tuple[int, int]
```

```expect
(none)
```

### case: the old spelling

```py python=3.9
from typing import Generic, TypeVar

T = TypeVar("T")


class Box(Generic[T]):
    pass
```

```expect
(none)
```

### case: no target

```py
class Box[T]:
    pass
```

```expect
(none)
```
