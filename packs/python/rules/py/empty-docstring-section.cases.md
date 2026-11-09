# py/empty-docstring-section — cases

### case: a numpy section with an underline and no content

```py
def f():
    """Do it.

    Returns
    -------

    """
```

```expect
4:5 py/empty-docstring-section
```
