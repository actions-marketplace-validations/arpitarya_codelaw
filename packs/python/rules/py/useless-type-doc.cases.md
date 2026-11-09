# py/useless-type-doc — cases

### case: an annotation is not read

```py
def f(a, _b: int):
    """Do it.

    Args:
        a: One.
    """
```

```expect
(none)
```

### case: Sphinx, the type beside the name

```py
def f(a, _b):
    """Do it.

    :param int a: One.
    :param str _b: Two.
    """
```

```expect
1:5 py/useless-type-doc
```
