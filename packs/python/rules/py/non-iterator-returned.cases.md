# py/non-iterator-returned — cases

### case: a list

```py
class Bag:
    def __iter__(self):
        return []
```

```expect
2:9 py/non-iterator-returned
```

### case: self without __next__

```py
class Bag:
    def __iter__(self):
        return self
```

```expect
2:9 py/non-iterator-returned
```

### case: self with __next__

```py
class Cursor:
    def __iter__(self):
        return self

    def __next__(self):
        return 1
```

```expect
(none)
```

### case: a generator

```py
class Bag:
    def __iter__(self):
        for i in range(3):
            yield i
```

```expect
(none)
```

### case: an instance of a non-iterator

```py
class Plain:
    pass


class Bag:
    def __iter__(self):
        return Plain()
```

```expect
6:9 py/non-iterator-returned
```

### case: an instance of an iterator

```py
class Cursor:
    def __next__(self):
        return 1


class Bag:
    def __iter__(self):
        return Cursor()
```

```expect
(none)
```

### case: the class itself

```py
class Plain:
    pass


class Bag:
    def __iter__(self):
        return Plain
```

```expect
6:9 py/non-iterator-returned
```

### case: a class with an iterator metaclass

```py
class Meta(type):
    def __next__(cls):
        return 1


class Plain(metaclass=Meta):
    pass


class Bag:
    def __iter__(self):
        return Plain
```

```expect
(none)
```

### case: iter() and an attribute

```py
class Bag:
    def __init__(self, f):
        self.f = f

    def __iter__(self):
        if self.f:
            return self.f
        return iter(())
```

```expect
(none)
```

### case: a mixed return

```py
class Bag:
    def __iter__(self):
        if self.x:
            return []
        return iter(self.x)
```

```expect
(none)
```

### case: a base the file imports

```py
from lib import Base


class Bag(Base):
    def __iter__(self):
        return self
```

```expect
(none)
```
