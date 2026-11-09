# py/docstring-extraneous-parameter — cases

### case: a function with **kwargs is not checked

```py
def area(width, **kwargs):
    """Return the area.

    Args:
        width: The width.
        height: The height.

    """
    return width
```

```expect
(none)
```
