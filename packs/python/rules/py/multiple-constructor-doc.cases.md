# py/multiple-constructor-doc — cases

### case: only the class documents them

```py
class A:
    """A.

    Args:
        x: One.
    """

    def __init__(self, x):
        """Init."""
```

```expect
(none)
```

### case: a constructor-named function outside a class

```py
def __init__(x):
    """Not a constructor.

    Args:
        x: One.
    """
```

```expect
(none)
```

### case: __new__

```py
class A:
    """A.

    Args:
        x: One.
    """

    def __new__(cls, x):
        """New.

        Args:
            x: One.
        """
```

```expect
1:7 py/multiple-constructor-doc
```
