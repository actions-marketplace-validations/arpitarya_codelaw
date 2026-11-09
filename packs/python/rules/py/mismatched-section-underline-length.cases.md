# py/mismatched-section-underline-length — cases

### case: too long

```py
def f(a):
    """Return a.

    Returns
    ---------
    int
        The value.

    """
```

```expect
5:5 py/mismatched-section-underline-length
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
