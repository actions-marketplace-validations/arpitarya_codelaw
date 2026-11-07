# py/boolean-type-hint-positional-argument — cases

### case: after *args, and a setter

```py
def f(*args, flag: bool):
    pass

class A:
    @x.setter
    def x(self, value: bool):
        pass
```

```expect
(none)
```
