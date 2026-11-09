# py/missing-yield-doc — cases

### case: the default accepts it

```py
def f():
    """Do it."""
    yield 1
```

```expect
(none)
```

### case: no docstring at all

```py options={"accept_no_yields_doc":false}
def f():
    yield 1
```

```expect
1:5 py/missing-yield-doc
```

### case: Sphinx reads returns

```py options={"accept_no_yields_doc":false}
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

### case: a type without a description

```py options={"accept_no_yields_doc":false}
def f():
    """Do it.

    Yields:
        int:
    """
    yield 1
```

```expect
1:5 py/missing-yield-doc
```

### case: yield from

```py options={"accept_no_yields_doc":false}
def f(xs):
    """Do it."""
    yield from xs
```

```expect
1:5 py/missing-yield-doc
```
