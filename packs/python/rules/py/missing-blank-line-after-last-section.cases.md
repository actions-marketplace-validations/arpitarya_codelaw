# py/missing-blank-line-after-last-section — cases

### case: closing quotes on the last line of text

```py
def f(a):
    """Return a.

    Args:
        a: The value."""
```

```expect
4:5 py/missing-blank-line-after-last-section
```

```fixed
def f(a):
    """Return a.

    Args:
        a: The value.

    """
```
