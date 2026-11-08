# py/super-init-not-called — cases

### case: no call to the base

```py
class Base:
    def __init__(self):
        self.ready = True


class Child(Base):
    def __init__(self):
        self.name = 'child'
```

```expect
7:9 py/super-init-not-called
```

### case: super() and Base.__init__

```py
class Base:
    def __init__(self):
        pass


class A(Base):
    def __init__(self):
        super().__init__()


class B(Base):
    def __init__(self):
        Base.__init__(self)
```

```expect
(none)
```

### case: the base has no __init__

```py
class Base:
    pass


class Child(Base):
    def __init__(self):
        self.name = 'child'
```

```expect
(none)
```

### case: an ancestor further up

```py
class Root:
    def __init__(self):
        pass


class Mid(Root):
    def __init__(self):
        super().__init__()


class Leaf(Mid):
    def __init__(self):
        self.x = 1
```

```expect
12:9 py/super-init-not-called
```
