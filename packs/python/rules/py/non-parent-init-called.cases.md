# py/non-parent-init-called — cases

### case: a class that is not a base

```py
class A:
    def __init__(self):
        self.a = 1


class B:
    def __init__(self):
        A.__init__(self)
```

```expect
8:9 py/non-parent-init-called
```

### case: a base and a base's base

```py
class A:
    def __init__(self):
        pass


class B(A):
    pass


class C(B):
    def __init__(self):
        A.__init__(self)
        B.__init__(self)
```

```expect
(none)
```

### case: an open ancestry

```py
from m import Mixin


class A:
    pass


class B(Mixin):
    def __init__(self):
        A.__init__(self)
```

```expect
(none)
```

### case: a class the file does not define

```py
class B:
    def __init__(self):
        Other.__init__(self)
```

```expect
(none)
```
