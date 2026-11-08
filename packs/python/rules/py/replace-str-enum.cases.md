# py/replace-str-enum — cases

### case: enum.Enum, the bases in either order

```py
import enum


class Color(enum.Enum, str):
    RED = "red"
```

```expect
4:1 py/replace-str-enum
```

```fixed
import enum


class Color(enum.StrEnum):
    RED = "red"
```

### case: a third base is reported, not fixed

```py
from enum import Enum


class Color(str, Mixin, Enum):
    RED = "red"
```

```expect
4:1 py/replace-str-enum
```

### case: an Enum not from enum, and an int enum, are fine

```py
from mylib import Enum
from enum import IntEnum


class A(str, Enum):
    pass


class B(int, IntEnum):
    pass
```

```expect
(none)
```
