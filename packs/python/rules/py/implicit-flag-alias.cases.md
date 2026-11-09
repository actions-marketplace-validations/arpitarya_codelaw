# py/implicit-flag-alias — cases

### case: literal unions

```py
from enum import IntFlag


class Perm(IntFlag):
    A = 1
    DUPLICATE = 1
    B = 2
    C = 3
    D = 5
    E = 8
    F = 17
    G = 17
    TRIPLICATE = 1
```

```expect
8:5 py/implicit-flag-alias
9:5 py/implicit-flag-alias
11:5 py/implicit-flag-alias
12:5 py/implicit-flag-alias
```

### case: disjoint flags and auto

```py
from enum import IntFlag, auto


class Perm(IntFlag):
    X = 1
    W = 2
    R = 4
    S = auto()
```

```expect
(none)
```

### case: explicit unions

```py
from enum import IntFlag


class Perm(IntFlag):
    X = 1
    W = 2
    R = 4
    RO = 4
    RW = R | W
```

```expect
(none)
```

### case: a multi-bit member declared first

```py
from enum import IntFlag


class Perm(IntFlag):
    A = 5
    B = 1
```

```expect
5:5 py/implicit-flag-alias
```

### case: two multi-bit members sharing a bit

```py
from enum import IntFlag


class Perm(IntFlag):
    A = 3
    B = 5
```

```expect
6:5 py/implicit-flag-alias
```

### case: a plain IntEnum

```py
from enum import IntEnum


class Level(IntEnum):
    LOW = 1
    HIGH = 3
```

```expect
(none)
```

### case: enum.Flag

```py
import enum


class Perm(enum.Flag):
    A = 1
    B = 2
    C = 3
```

```expect
7:5 py/implicit-flag-alias
```
