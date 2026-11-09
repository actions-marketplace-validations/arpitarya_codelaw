# py/over-indentation — cases

### case: one line at the right depth means no line is reported

```py
def f():
    """Return one.

    Normal.
        Indented example.
    """
```

```expect
(none)
```

### case: the closing quotes over-indented

```py
def f():
    """Return one.

    Text.
        """
```

```expect
5:1 py/over-indentation
```

```fixed
def f():
    """Return one.

    Text.
    """
```
