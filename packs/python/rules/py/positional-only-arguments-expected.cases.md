# py/positional-only-arguments-expected — cases

### case: keywords for positional-only parameters

```py
def area(width, height, /):
    return width * height


area(width=2, height=3)
```

```expect
5:1 py/positional-only-arguments-expected
```

### case: one positional-only and one ordinary

```py
def f(a, /, b):
    pass


f(1, b=2)
f(a=1, b=2)
```

```expect
6:1 py/positional-only-arguments-expected
```

### case: by position

```py
def area(width, height, /):
    return 1


area(2, 3)
```

```expect
(none)
```

### case: **kwargs absorbs

```py
def f(a, /, **kw):
    pass


f(1, a=2)
```

```expect
(none)
```

### case: not positional-only

```py
def f(a, b):
    pass


f(a=1, b=2)
```

```expect
(none)
```
