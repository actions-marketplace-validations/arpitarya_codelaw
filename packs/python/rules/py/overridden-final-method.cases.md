# py/overridden-final-method — cases

### case: an override

```py
from typing import final


class Base:
    @final
    def run(self): ...


class Child(Base):
    def run(self): ...
```

```expect
10:9 py/overridden-final-method
```

### case: a method that is not final

```py
class Base:
    def run(self): ...


class Child(Base):
    def run(self): ...
```

```expect
(none)
```

### case: another method

```py
from typing import final


class Base:
    @final
    def run(self): ...


class Child(Base):
    def stop(self): ...
```

```expect
(none)
```
