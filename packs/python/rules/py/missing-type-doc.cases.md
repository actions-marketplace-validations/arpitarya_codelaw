# py/missing-type-doc — cases

### case: an annotation is a type

```py
def f(x: int, y):
    """Do it.

    Args:
        x: One.
        y (str): Two.
    """
```

```expect
(none)
```

### case: NumPy style, a type missing

```py
def f(x, y):
    """Do it.

    Parameters
    ----------
    x : int
        One.
    y
        Two.
    """
```

```expect
1:5 py/missing-type-doc
```

### case: a starred parameter needs no type

```py
def f(x: int, *args):
    """Do it.

    Args:
        x: One.
        *args: The rest.
    """
```

```expect
(none)
```
