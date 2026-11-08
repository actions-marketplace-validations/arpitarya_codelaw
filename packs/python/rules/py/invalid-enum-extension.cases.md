# py/invalid-enum-extension — cases

### case: extending an enum with members

```py
from enum import Enum


class Color(Enum):
    RED = 1


class Shade(Color):
    DARK = 2
```

```expect
8:7 py/invalid-enum-extension
```

### case: an enum without members as a base

```py
from enum import Enum


class Base(Enum):
    def label(self):
        return self.name


class Color(Base):
    RED = 1
```

```expect
(none)
```

### case: a class that is not an enum

```py
class Color:
    RED = 1


class Shade(Color):
    DARK = 2
```

```expect
(none)
```

### case: an enum derived from an enum with members through IntEnum

```py
import enum


class Num(enum.IntEnum):
    ONE = 1


class More(Num):
    TWO = 2
```

```expect
8:7 py/invalid-enum-extension
```
