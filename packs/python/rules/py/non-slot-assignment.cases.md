# py/non-slot-assignment — cases

### case: a property setter and a base class

```py
class A:
    __slots__ = ("_a",)

    @property
    def a(self):
        return self._a

    @a.setter
    def a(self, value):
        self._a = value

    def __init__(self, a):
        self.a = a


class B(A):
    __slots__ = ("b",)

    def __init__(self):
        self.c = 1
```

```expect
(none)
```

### case: an augmented assignment

```py
class A:
    __slots__ = ("a",)

    def __init__(self):
        self.a = 0
        self.n += 1
```

```expect
6:9 py/non-slot-assignment
```
