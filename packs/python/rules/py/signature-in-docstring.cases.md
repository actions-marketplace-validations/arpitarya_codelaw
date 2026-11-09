# py/signature-in-docstring — cases

### case: a name inside a word is not a call

```py
def add(a, b):
    """Run the padd(x) helper."""
```

```expect
(none)
```

### case: the name after a space and a parenthesis

```py
def add(a, b):
    """Call add(a, b) to sum."""
```

```expect
2:5 py/signature-in-docstring
```
