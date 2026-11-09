# py/missing-format-attribute — cases

### case: an int

```py
"{0.missing}".format(2)
```

```expect
1:1 py/missing-format-attribute
```

### case: a keyword argument

```py
"{0:{a.x}}".format(1, a=2)
```

```expect
1:1 py/missing-format-attribute
```

### case: a str

```py
"{a.nope}".format(a="x")
```

```expect
1:1 py/missing-format-attribute
```

### case: an attribute the type has

```py
a = "{0.real} {1.upper} {2.append} {x.items}".format(1, "a", [], x={})
```

```expect
(none)
```

### case: a dunder

```py
a = "{0.__class__}".format(1)
```

```expect
(none)
```

### case: an empty attribute name

```py
a = "There are {.:2f} errors.".format(1)
```

```expect
(none)
```

### case: a name bound once

```py
def f():
    n = 2
    return "{0.missing}".format(n)
```

```expect
3:12 py/missing-format-attribute
```

### case: an object the file does not state

```py
def f(obj):
    return "{0.missing}".format(obj)
```

```expect
(none)
```

### case: starred arguments

```py
def f(args):
    return "{0.missing}".format(*args)
```

```expect
(none)
```
