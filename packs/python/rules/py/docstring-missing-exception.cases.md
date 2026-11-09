# py/docstring-missing-exception — cases

### case: NotImplementedError is not required

```py
def f():
    """Do it."""
    raise NotImplementedError
```

```expect
(none)
```

### case: a re-raise inside except

```py
def f():
    """Do it."""
    try:
        run()
    except OSError:
        raise
```

```expect
2:5 py/docstring-missing-exception
```
