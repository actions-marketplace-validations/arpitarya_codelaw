# py/missing-raises-doc — cases

### case: a documented parent covers its children

```py
def f():
    """Do it.

    Raises:
        Exception: Always.
    """
    raise ValueError("x")
```

```expect
(none)
```

### case: a class of the file derived from a documented one

```py
class E(NameError):
    pass


def f():
    """Do it.

    Raises:
        NameError: Never.
    """
    raise E()
```

```expect
(none)
```

### case: a documented child does not cover its parent

```py
def f():
    """Do it.

    Raises:
        ValueError: Never.
    """
    raise Exception("x")
```

```expect
1:5 py/missing-raises-doc
```

### case: an exception the function catches

```py
def f():
    """Do it.

    Raises:
        NameError: Never.
    """
    try:
        raise RuntimeError("x")
    except RuntimeError:
        pass
    raise NameError("y")
```

```expect
(none)
```

### case: a rethrown exception

```py
def f():
    """Do it.

    Raises:
        NameError: Never.
    """
    try:
        g()
    except (RuntimeError, ValueError):
        raise
```

```expect
1:5 py/missing-raises-doc
```

### case: Sphinx style

```py
def f():
    """Do it.

    :raises NameError: Never.
    """
    raise RuntimeError("x")
```

```expect
1:5 py/missing-raises-doc
```

### case: a docstring of no known style is accepted by default

```py
def f():
    """Do it."""
    raise RuntimeError("x")
```

```expect
(none)
```

### case: a docstring of no known style, strict

```py options={"accept_no_raise_doc":false}
def f():
    """Do it."""
    raise RuntimeError("x")
```

```expect
1:5 py/missing-raises-doc
```

### case: a function without a docstring is not asked

```py options={"accept_no_raise_doc":false}
def f():
    raise RuntimeError("x")
```

```expect
(none)
```

### case: a property setter reads its getter

```py
class A:
    @property
    def v(self):
        """The value.

        Raises:
            RuntimeError: Always.
        """
        raise RuntimeError()

    @v.setter
    def v(self, value):
        raise AttributeError()
```

```expect
3:9 py/missing-raises-doc
```

### case: a name the checker cannot place is not read

```py
from somewhere import Oops


def f():
    """Do it.

    Raises:
        NameError: Never.
    """
    raise Oops()
```

```expect
(none)
```
