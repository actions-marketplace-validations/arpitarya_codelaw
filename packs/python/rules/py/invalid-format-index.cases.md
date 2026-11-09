# py/invalid-format-index — cases

### case: an index past the end of a list

```py
"{[0][1]}".format(["a"])
```

```expect
1:1 py/invalid-format-index
```

### case: an index of an int inside a tuple

```py
"{[0][0]}".format(((1, )))
```

```expect
1:1 py/invalid-format-index
```

### case: an int argument

```py
"{a[0]}".format(a=1)
```

```expect
1:1 py/invalid-format-index
```

### case: the object class

```py
"{a[0]}".format(a=object)
```

```expect
1:1 py/invalid-format-index
```

### case: a string literal

```py
"{0[5]}".format("abc")
```

```expect
1:1 py/invalid-format-index
```

### case: a dict literal

```py
"{0[missing]}".format({"a": 1})
ok = "{0[a]}".format({"a": 1})
```

```expect
1:1 py/invalid-format-index
```

### case: a valid index

```py
a = "{0[0]} {1[1]} {x[a]}".format([1], (1, 2), x={"a": 1})
```

```expect
(none)
```

### case: a name bound once

```py
def f():
    data = ["a"]
    return "{0[2]}".format(data)
```

```expect
3:12 py/invalid-format-index
```

### case: a call the file does not state

```py
def f(g):
    return "{0[2]}".format(g())
```

```expect
(none)
```

### case: starred arguments

```py
def f(args):
    return "{0[2]}".format(*args)
```

```expect
(none)
```
