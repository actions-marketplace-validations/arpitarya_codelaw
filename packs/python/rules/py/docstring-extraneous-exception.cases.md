# py/docstring-extraneous-exception — cases

### case: raised by name

```py
def divide(a, b):
    """Divide a by b.

    Raises:
        ValueError: If b is zero.

    """
    raise ValueError("b")
```

```expect
(none)
```
