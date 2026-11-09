# py/missing-any-param-doc — cases

### case: the default accepts it

```py
def f(x, y):
    """Do it."""
```

```expect
(none)
```

### case: a function without a docstring is not asked

```py options={"accept_no_param_doc":false}
def f(x, y):
    return x
```

```expect
(none)
```

### case: a pointer to another docstring

```py options={"accept_no_param_doc":false}
def f(x, y):
    """Do it.

    For the parameters, see :func:`g`.
    """
```

```expect
(none)
```

### case: annotated parameters are asked for their description

```py options={"accept_no_param_doc":false}
def f(x: int, y: int):
    """Do it."""
```

```expect
(none)
```
