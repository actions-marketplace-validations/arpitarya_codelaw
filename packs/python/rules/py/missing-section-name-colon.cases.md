# py/missing-section-name-colon — cases

### case: trailing spaces and no colon

```py
def f(a):
    """Return a.

    Args  
        a: The value.

    """
```

```expect
4:5 py/missing-section-name-colon
```

```fixed
def f(a):
    """Return a.

    Args:
        a: The value.

    """
```
