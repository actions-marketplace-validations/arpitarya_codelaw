# py/arguments-renamed — cases

### case: a renamed parameter

```py
class Base:
    def save(self, path):
        pass


class Child(Base):
    def save(self, target):
        pass
```

```expect
7:9 py/arguments-renamed
```

### case: the same names

```py
class Base:
    def save(self, path):
        pass


class Child(Base):
    def save(self, path):
        pass
```

```expect
(none)
```

### case: a different count is arguments-differ

```py
class Base:
    def save(self, path):
        pass


class Child(Base):
    def save(self):
        pass
```

```expect
(none)
```

### case: an underscore name

```py
class Base:
    def save(self, _path):
        pass


class Child(Base):
    def save(self, path):
        pass
```

```expect
(none)
```
