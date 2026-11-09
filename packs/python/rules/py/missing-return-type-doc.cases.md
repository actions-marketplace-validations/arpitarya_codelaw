# py/missing-return-type-doc — cases

### case: an annotation is a type

```py options={"accept_no_return_doc":false}
def f() -> int:
    """Do it."""
    return 1
```

```expect
(none)
```

### case: Sphinx

```py options={"accept_no_return_doc":false}
def f():
    """Do it.

    :returns: One.
    """
    return 1
```

```expect
1:5 py/missing-return-type-doc
```

### case: NumPy, a type line

```py options={"accept_no_return_doc":false}
def f():
    """Do it.

    Returns
    -------
    int
        One.
    """
    return 1
```

```expect
(none)
```

### case: a property with its type in the first line

```py options={"accept_no_return_doc":false}
class A:
    @property
    def v(self):
        """int: The value."""
        return 1
```

```expect
(none)
```

### case: a method's first line is not a type

```py options={"accept_no_return_doc":false}
class A:
    def v(self):
        """int: The value."""
        return 1
```

```expect
2:9 py/missing-return-type-doc
```
