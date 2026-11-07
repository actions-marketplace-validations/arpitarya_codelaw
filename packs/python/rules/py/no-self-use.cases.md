# py/no-self-use — cases

### case: uses of self, stubs, decorators, dunders and super are fine

```py
class A:
    def a(self):
        return self.name

    def b(self):
        return [self for _ in range(2)]

    def c(self):
        """Doc."""
        raise NotImplementedError

    def d(self):
        msg = "subclass it"
        raise NotImplementedError(msg)

    @property
    def e(self):
        return 1

    def __repr__(self):
        return "A"

    def f(self):
        super().f()

    def g(self):
        ...
```

```expect
(none)
```

### case: a module function is not a method

```py
def h(self):
    return 1
```

```expect
(none)
```
