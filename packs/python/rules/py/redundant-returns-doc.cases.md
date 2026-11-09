# py/redundant-returns-doc — cases

### case: Sphinx, a generator

```py
def f():
    """Do it.

    :returns: One.
    :rtype: int
    """
    yield 1
```

```expect
(none)
```

### case: Google, a generator

```py
def f():
    """Do it.

    Returns:
        int: One.
    """
    yield 1
```

```expect
1:5 py/redundant-returns-doc
```

### case: Sphinx, an rtype only

```py
def f():
    """Do it.

    :rtype: int
    """
    return None
```

```expect
1:5 py/redundant-returns-doc
```

### case: an abstract method

```py
import abc


class A:
    @abc.abstractmethod
    def f(self):
        """Do it.

        Returns:
            int: One.
        """
        raise NotImplementedError
```

```expect
(none)
```

### case: a value on one path

```py
def f(x):
    """Do it.

    Returns:
        int: One.
    """
    if x:
        return None
    return 1
```

```expect
(none)
```
