# py/type-alias-without-annotation — cases

### case: a union of names and None, in a class

```py path=api.pyi
class Config:
    Key = str | int | None
```

```expect
2:5 py/type-alias-without-annotation
```

```fixed
from typing import TypeAlias

class Config:
    Key: TypeAlias = str | int | None
```

### case: values that are not types are fine

```py path=api.pyi
import enum

A = 1 | 2
B = make()
C: int
__all__ = ["A"]


class Color(enum.Enum):
    NONE = None


def f() -> None:
    D = list[int]
```

```expect
(none)
```
