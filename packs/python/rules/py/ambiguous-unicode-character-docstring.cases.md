# py/ambiguous-unicode-character-docstring — cases

### case: a module docstring

```py
"""Dо it."""
```

```expect
1:5 py/ambiguous-unicode-character-docstring
```

### case: a function docstring, a multiplication sign

```py
def f():
    """Return x × 2."""
```

```expect
2:17 py/ambiguous-unicode-character-docstring
```

### case: a string that is not a docstring is the string rule's

```py
def f():
    x = "Don’t"
```

```expect
(none)
```

### case: a docstring of another script is left alone

```py
def f():
    """Привет."""
```

```expect
(none)
```
