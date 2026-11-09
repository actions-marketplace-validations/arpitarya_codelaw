# py/useless-param-doc — cases

### case: a name that only mentions the parameter

```py
def f(a, _b):
    """Do it, as _b would.

    Args:
        a: One.
    """
```

```expect
(none)
```

### case: a NumPy entry

```py
def f(a, _b):
    """Do it.

    Parameters
    ----------
    a : int
        One.
    _b : int
        Two.
    """
```

```expect
1:5 py/useless-param-doc
```
