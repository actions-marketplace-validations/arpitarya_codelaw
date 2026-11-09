# py/arguments-differ — cases

### case: fewer parameters

```py
class Base:
    def save(self, path, mode):
        pass


class Child(Base):
    def save(self, path):
        pass
```

```expect
7:9 py/arguments-differ
```

### case: an extra required parameter

```py
class Base:
    def save(self, path):
        pass


class Child(Base):
    def save(self, path, mode):
        pass
```

```expect
7:9 py/arguments-differ
```

### case: an extra parameter with a default

```py
class Base:
    def save(self, path):
        pass


class Child(Base):
    def save(self, path, mode=None):
        pass
```

```expect
(none)
```

### case: *args and **kwargs

```py
class Base:
    def save(self, path, mode):
        pass


class A(Base):
    def save(self, *args):
        pass


class B(Base):
    def save(self, path, mode, **kw):
        pass
```

```expect
(none)
```

### case: a missing keyword-only parameter

```py
class Base:
    def run(self, *, fast):
        pass


class Child(Base):
    def run(self):
        pass
```

```expect
7:9 py/arguments-differ
```

### case: __init__ is not read

```py
class Base:
    def __init__(self, a, b):
        pass


class Child(Base):
    def __init__(self, a):
        pass
```

```expect
(none)
```

### case: an unrelated method

```py
class Base:
    def a(self, x):
        pass


class Child(Base):
    def b(self):
        pass
```

```expect
(none)
```

### case: annotated *args and **kwargs

A `**kwargs: T` or `*args: T` is a `typed_parameter` wrapping the splat in the grammar; it is not a positional parameter (found by the corpus run: Flask's `MethodView.dispatch_request(self, **kwargs: t.Any)`).

```py
class Base:
    def run(self):
        pass

    def save(self, path, mode):
        pass


class A(Base):
    def run(self, **kwargs: int):
        pass

    def save(self, *args: int):
        pass


class B(Base):
    def save(self, path, mode, **kw: int):
        pass
```

```expect
(none)
```
