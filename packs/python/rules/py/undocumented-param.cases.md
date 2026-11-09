# py/undocumented-param — cases

### case: self and underscore names are skipped

```py
class A:
    def add(self, a, _b):
        """Add.

        Args:
            a: The first number.

        """
```

```expect
(none)
```

### case: a variadic parameter

```py
def f(a, *args, **kwargs):
    """Call.

    Args:
        a: The first.
        *args: Positional.

    """
```

```expect
1:5 py/undocumented-param
```

### case: NumPy style

```py
def f(a, b):
    """Call.

    Parameters
    ----------
    a : int
        The first.

    """
```

```expect
1:5 py/undocumented-param
```
