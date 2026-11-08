# py/bad-super-call — cases

### case: another class of the file

```py
class Base:
    def run(self):
        pass


class Child(Base):
    def run(self):
        super(Base, self).run()
```

```expect
8:15 py/bad-super-call
```

```fixed
class Base:
    def run(self):
        pass


class Child(Base):
    def run(self):
        super(Child, self).run()
```

### case: the enclosing class and no arguments

```py
class Base:
    pass


class Child(Base):
    def run(self):
        super(Child, self).run()
        super().run()
```

```expect
(none)
```

### case: a class the file does not define

```py
class Child(Base):
    def run(self):
        super(Other, self).run()
```

```expect
(none)
```
