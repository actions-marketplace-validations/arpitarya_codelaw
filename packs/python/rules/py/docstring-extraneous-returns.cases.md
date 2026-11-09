# py/docstring-extraneous-returns — cases

### case: a value returned

```py
def log(message):
    """Write a message.

    Returns:
        The text.

    """
    return message
```

```expect
(none)
```
