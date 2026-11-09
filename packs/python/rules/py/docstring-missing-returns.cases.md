# py/docstring-missing-returns — cases

### case: a property is not checked

```py
class A:
    @property
    def size(self):
        """The size."""
        return 1
```

```expect
(none)
```

### case: a bare return

```py
def f(a):
    """Do it."""
    if a:
        return
```

```expect
(none)
```
