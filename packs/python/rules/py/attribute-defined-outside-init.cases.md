# py/attribute-defined-outside-init — cases

### case: set by a helper __init__ calls, declared in the class, a setter, a subclass, a mixin

```py
class A:
    count: int

    def __init__(self):
        self._reset()

    def _reset(self):
        self.items = []

    def bump(self):
        self.count = 1

    @property
    def name(self):
        return self._name

    @name.setter
    def name(self, v):
        self._name = v


class B(A):
    def go(self):
        self.extra = 1


class LoggingMixin:
    def log(self):
        self.lines = []
```

```expect
(none)
```

### case: a test class sets in setUp; a plain class with an explicit object base

```py
class T:
    def setUp(self):
        self.client = 1

    def test_a(self):
        self.client = 2


class C(object):
    def run(self):
        self.done = True
```

```expect
11:9 py/attribute-defined-outside-init
```
