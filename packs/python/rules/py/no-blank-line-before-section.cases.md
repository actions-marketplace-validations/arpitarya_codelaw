# py/no-blank-line-before-section — cases

### case: the first section after the summary

```py
def f(a):
    """Return a.
    Args:
        a: The value.

    """
```

```expect
3:5 py/no-blank-line-before-section
```

```fixed
def f(a):
    """Return a.

    Args:
        a: The value.

    """
```
