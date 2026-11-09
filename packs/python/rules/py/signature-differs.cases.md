# py/signature-differs — cases

### case: a default the override drops

```py
class Base:
    def run(self, retries=3, delay=None):
        pass


class Child(Base):
    def run(self, retries, delay=None):
        pass
```

```expect
7:9 py/signature-differs
```

### case: the same defaults

```py
class Base:
    def run(self, retries=3):
        pass


class Child(Base):
    def run(self, retries=3):
        pass
```

```expect
(none)
```

### case: a default the override adds

```py
class Base:
    def run(self, retries):
        pass


class Child(Base):
    def run(self, retries=3):
        pass
```

```expect
(none)
```

### case: fewer parameters is arguments-differ

```py
class Base:
    def run(self, retries=3, delay=None):
        pass


class Child(Base):
    def run(self, retries=3):
        pass
```

```expect
(none)
```

### case: variadics

```py
class Base:
    def run(self, retries=3):
        pass


class Child(Base):
    def run(self, *args, **kwargs):
        pass
```

```expect
(none)
```

### case: a base the file imports

```py
from lib import Base


class Child(Base):
    def run(self, retries):
        pass
```

```expect
(none)
```

### case: a static method

```py
class Base:
    @staticmethod
    def run(retries=3):
        pass


class Child(Base):
    @staticmethod
    def run(retries):
        pass
```

```expect
(none)
```
