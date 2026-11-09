# py/match-class-positional-attributes — cases

### case: positional patterns

```py
class A:
    __match_args__ = ("x", "y")


def f(v):
    match v:
        case A(1):
            pass
        case A(1, 2):
            pass
        case A(x=1, y=2):
            pass
```

```expect
7:14 py/match-class-positional-attributes
9:14 py/match-class-positional-attributes
```

```fixed
class A:
    __match_args__ = ("x", "y")


def f(v):
    match v:
        case A(x=1):
            pass
        case A(x=1, y=2):
            pass
        case A(x=1, y=2):
            pass
```

### case: a positional pattern with a keyword one

```py
class A:
    __match_args__ = ("x", "y")


def f(v):
    match v:
        case A(1, y=2):
            pass
```

```expect
7:14 py/match-class-positional-attributes
```

```fixed
class A:
    __match_args__ = ("x", "y")


def f(v):
    match v:
        case A(x=1, y=2):
            pass
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
```

```expect
12:14 py/match-class-positional-attributes
```

```fixed
from dataclasses import dataclass


@dataclass
class P:
    x: int
    y: int


def f(v):
    match v:
        case P(x=1, y=2):
            pass
```

### case: a builtin and a NamedTuple

```py
from typing import NamedTuple


class R(NamedTuple):
    x: int
    y: int


def f(v):
    match v:
        case int(2):
            pass
        case bool(True):
            pass
        case R(1, 2):
            pass
```

```expect
(none)
```

### case: no __match_args__ the file can see

```py
from lib import C


class A:
    pass


def f(v):
    match v:
        case C(1):
            pass
        case A(1):
            pass
```

```expect
(none)
```
