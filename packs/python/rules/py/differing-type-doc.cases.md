# py/differing-type-doc — cases

### case: Google style

```py
def f(a):
    """Do it.

    Args:
        b (int): Two.
    """
```

```expect
1:5 py/differing-type-doc
```

### case: an embedded type

```py
def f(a):
    """Do it.

    :param int b: Two.
    """
```

```expect
1:5 py/differing-type-doc
```
