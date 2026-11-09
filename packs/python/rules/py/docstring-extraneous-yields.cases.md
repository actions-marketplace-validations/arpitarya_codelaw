# py/docstring-extraneous-yields — cases

### case: a generator

```py
def count(n):
    """Count.

    Yields:
        The next number.

    """
    yield n
```

```expect
(none)
```
