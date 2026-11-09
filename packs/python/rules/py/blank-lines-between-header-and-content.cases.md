# py/blank-lines-between-header-and-content — cases

### case: numpy: blank lines after the underline

```py
def f(a):
    """Return a.

    Parameters
    ----------

    a : int
        The value.

    """
```

```expect
4:5 py/blank-lines-between-header-and-content
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

### case: one blank line before a directive is kept

```py
def f():
    """Run.

    Example:

    .. code-block:: python

        f()

    """
```

```expect
(none)
```
