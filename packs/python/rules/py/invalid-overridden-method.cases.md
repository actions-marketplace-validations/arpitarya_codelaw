# py/invalid-overridden-method — cases

### case: a plain def over an async def

```py
class Base:
    async def fetch(self): ...


class Child(Base):
    def fetch(self): ...
```

```expect
6:9 py/invalid-overridden-method
```

### case: a method over a property

```py
class Base:
    @property
    def size(self):
        return 1


class Child(Base):
    def size(self):
        return 2
```

```expect
8:9 py/invalid-overridden-method
```

### case: a property over a method

```py
class Base:
    def size(self):
        return 1


class Child(Base):
    @property
    def size(self):
        return 2
```

```expect
8:9 py/invalid-overridden-method
```

### case: the same kind

```py
class Base:
    async def fetch(self): ...

    @property
    def size(self):
        return 1


class Child(Base):
    async def fetch(self): ...

    @property
    def size(self):
        return 2
```

```expect
(none)
```
