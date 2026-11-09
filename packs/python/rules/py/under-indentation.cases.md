# py/under-indentation — cases

### case: a class docstring

```py
class A:
    """One.

 Two.
    """
```

```expect
4:1 py/under-indentation
```

```fixed
class A:
    """One.

    Two.
    """
```

### case: a continuation line after a backslash, and blank lines, are not read

```py
def f():
    """One \
two.

    Three.
    """
```

```expect
(none)
```
