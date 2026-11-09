# py/declare-non-slot — cases

### case: an annotation outside the slots

```py
class Point:
    __slots__ = ("x", "y")

    x: int
    y: int
    label: str
```

```expect
6:5 py/declare-non-slot
```

### case: a name in a base's slots

```py
class Base:
    __slots__ = ("a", "b")


class Child(Base):
    __slots__ = ("c",)

    a: int
    d: int
```

```expect
9:5 py/declare-non-slot
```

### case: __dict__ in the slots

```py
class Point:
    __slots__ = ("x", "__dict__")

    x: int
    label: str
```

```expect
(none)
```

### case: a ClassVar

```py
from typing import ClassVar
import typing


class Point:
    __slots__ = ("x",)

    x: int
    kind: ClassVar[str]
    other: typing.ClassVar[int]
    bare: ClassVar
```

```expect
(none)
```

### case: an annotation with a value

```py
class Point:
    __slots__ = ("x",)

    x: int
    label: str = "p"
```

```expect
(none)
```

### case: a base without slots

```py
class Base:
    pass


class Point(Base):
    __slots__ = ("x",)

    price: int
```

```expect
(none)
```

### case: empty slots

```py
class Point:
    __slots__ = ()

    a: int
```

```expect
(none)
```
