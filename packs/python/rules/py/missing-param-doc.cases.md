# py/missing-param-doc — cases

### case: every parameter documented

```py
def f(x, y):
    """Do it.

    :param x: One.
    :param y: Two.
    """
```

```expect
(none)
```

### case: a docstring that documents none is accepted

```py
def f(x, y):
    """Do it."""
```

```expect
(none)
```

### case: Google style

```py
def f(x, y):
    """Do it.

    Args:
        x: One.
    """
```

```expect
1:5 py/missing-param-doc
```

### case: NumPy style

```py
def f(x, y):
    """Do it.

    Parameters
    ----------
    x : int
        One.
    """
```

```expect
1:5 py/missing-param-doc
```

### case: a sentence that points elsewhere

```py
def f(x, y):
    """Do it.

    :param x: One.

    For the other parameters, see :func:`g`.
    """
```

```expect
(none)
```

### case: self, a private name and a starred parameter

```py
class A:
    def f(self, x, _y, *args):
        """Do it.

        :param x: One.
        :param args: The rest.
        """
```

```expect
(none)
```

### case: a private function is skipped

```py
def _f(x, y):
    """Do it.

    :param x: One.
    """
```

```expect
(none)
```

### case: a private function checked on request

```py options={"check_private_functions":true}
def _f(x, y):
    """Do it.

    :param x: One.
    """
```

```expect
1:5 py/missing-param-doc
```

### case: a constructor is read against its class docstring

```py options={"check_private_functions":true}
class A:
    """A.

    Args:
        y: Two.
    """

    def __init__(self, x, y):
        pass
```

```expect
1:7 py/missing-param-doc
```
