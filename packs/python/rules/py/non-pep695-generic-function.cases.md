# py/non-pep695-generic-function — cases

### case: two variables and a constraint

```py
from typing import TypeVar

K = TypeVar("K")
V = TypeVar("V", int, str)


def get(d: dict[K, V], k: K) -> V:
    return d[k]
```

```expect
7:5 py/non-pep695-generic-function
```

```fixed
from typing import TypeVar

K = TypeVar("K")
V = TypeVar("V", int, str)


def get[K, V: (int, str)](d: dict[K, V], k: K) -> V:
    return d[k]
```

### case: a method is left alone

```py
from typing import TypeVar

T = TypeVar("T")


class A:
    def f(self, x: T) -> T:
        return x
```

```expect
(none)
```

### case: a variable only in the return type

```py
from typing import TypeVar

T = TypeVar("T")


def make() -> T: ...
```

```expect
(none)
```
