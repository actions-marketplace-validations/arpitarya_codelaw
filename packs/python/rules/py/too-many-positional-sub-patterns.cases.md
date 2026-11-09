# py/too-many-positional-sub-patterns — cases

### case: more than __match_args__

```py
class A:
    __match_args__ = ("x",)


def f(v):
    match v:
        case A(1):
            pass
        case A(1, 2):
            pass
```

```expect
9:14 py/too-many-positional-sub-patterns
```

### case: an own tuple wins over the base's

```py
class A:
    __match_args__ = ("x",)


class B(A):
    __match_args__ = ("x", "y")


def f(v):
    match v:
        case B(1, 2):
            pass
        case B(1, 2, 3):
            pass
```

```expect
13:14 py/too-many-positional-sub-patterns
```

### case: a builtin

```py
def f(v):
    match v:
        case int(1):
            pass
        case int(1, 2):
            pass
        case tuple((1, 2)):
            pass
```

```expect
5:14 py/too-many-positional-sub-patterns
```

### case: a dataclass

```py
from dataclasses import dataclass


@dataclass
class P:
    x: int
    y: int


def f(v):
    match v:
        case P(1, 2):
            pass
        case P(1, 2, 3):
            pass
```

```expect
14:14 py/too-many-positional-sub-patterns
```

### case: keyword patterns

```py
class A:
    __match_args__ = ("x",)


def f(v):
    match v:
        case A(1, y=2):
            pass
```

```expect
(none)
```

### case: an invalid __match_args__

```py
class A:
    __match_args__ = ["x"]


def f(v):
    match v:
        case A(1, 2):
            pass
```

```expect
(none)
```

### case: a class the file does not define

```py
from lib import C


def f(v):
    match v:
        case C(1, 2, 3):
            pass
```

```expect
(none)
```

### case: a NamedTuple

```py
from typing import NamedTuple


class R(NamedTuple):
    x: int
    y: int


def f(v):
    match v:
        case R(1, 2):
            pass
```

```expect
(none)
```
