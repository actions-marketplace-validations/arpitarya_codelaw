# py/missing-dashed-underline-after-section — cases

### case: an underline of equal signs

```py
def f(a):
    """Return a.

    Parameters
    ==========
    a : int
        The value.

    """
```

```expect
5:5 py/missing-dashed-underline-after-section
```

```fixed
def f(a):
    """Return a.

    Parameters
    ----------
    a : int
        The value.

    """
```

### case: Google style needs none

```py
def f(a):
    """Return a.

    Args:
        a: The value.

    """
```

```expect
(none)
```
