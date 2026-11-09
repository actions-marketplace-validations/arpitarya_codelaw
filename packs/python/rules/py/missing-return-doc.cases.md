# py/missing-return-doc — cases

### case: the default accepts it

```py
def f():
    """Do it."""
    return 1
```

```expect
(none)
```

### case: no docstring at all

```py options={"accept_no_return_doc":false}
def f():
    return 1
```

```expect
1:5 py/missing-return-doc
```

### case: a bare return

```py options={"accept_no_return_doc":false}
def f():
    """Do it."""
    return
```

```expect
(none)
```

### case: return None

```py options={"accept_no_return_doc":false}
def f():
    """Do it."""
    return None
```

```expect
(none)
```

### case: a type without a description

```py options={"accept_no_return_doc":false}
def f():
    """Do it.

    Returns:
        bool:
    """
    return True
```

```expect
1:5 py/missing-return-doc
```

### case: Sphinx, only a type

```py options={"accept_no_return_doc":false}
def f():
    """Do it.

    :rtype: bool
    """
    return True
```

```expect
1:5 py/missing-return-doc
```

### case: a property with a summary

```py options={"accept_no_return_doc":false}
class A:
    @property
    def v(self):
        """The value."""
        return 1
```

```expect
(none)
```

### case: a nested function's return is its own

```py options={"accept_no_return_doc":false}
def f():
    """Do it."""

    def g():
        return 1
```

```expect
4:9 py/missing-return-doc
```

### case: a private function is skipped

```py options={"accept_no_return_doc":false}
def _f():
    return 1
```

```expect
(none)
```
