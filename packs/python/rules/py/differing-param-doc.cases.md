# py/differing-param-doc — cases

### case: a keyword-only parameter

```py
def f(*, key):
    """Do it.

    Args:
        key: One.
        other: Two.
    """
```

```expect
1:5 py/differing-param-doc
```

### case: keyword arguments documented in their own section

```py
def f(a, b=1):
    """Do it.

    Args:
        a: One.

    Keyword Args:
        b: Two.
    """
```

```expect
(none)
```

### case: the stars may be written or left out

```py
def f(*args, **kwargs):
    """Do it.

    Args:
        *args: One.
        kwargs: Two.
    """
```

```expect
(none)
```
