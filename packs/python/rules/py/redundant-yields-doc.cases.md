# py/redundant-yields-doc — cases

### case: a yield in a nested function is not this function's

```py
def f():
    """Do it.

    Yields:
        int: One.
    """

    def g():
        yield 1
```

```expect
1:5 py/redundant-yields-doc
```

### case: a generator

```py
def f():
    """Do it.

    Yields:
        int: One.
    """
    yield 1
```

```expect
(none)
```
