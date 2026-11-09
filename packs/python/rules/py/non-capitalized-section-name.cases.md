# py/non-capitalized-section-name — cases

### case: a capitalized section

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

### case: a numpy section

```py
def f(a):
    """Return a.

    returns
    -------
    int
        The value.

    """
```

```expect
4:5 py/non-capitalized-section-name
```

```fixed
def f(a):
    """Return a.

    Returns
    -------
    int
        The value.

    """
```
