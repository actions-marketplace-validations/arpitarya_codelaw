# py/redefined-slots-in-subclass — cases

### case: a grandparent's slot

```py
class A:
    __slots__ = ("x",)


class B(A):
    pass


class C(B):
    __slots__ = ("x",)
```

```expect
10:18 py/redefined-slots-in-subclass
```

### case: a base from another file is not read

```py
from lib import Base


class Child(Base):
    __slots__ = ("name",)
```

```expect
(none)
```
