# py/repeated-append — cases

### case: a name not known to be a list

```py
names = make()
names.append("a")
names.append("b")
```

```expect
(none)
```

### case: another list in between

```py
a = []
b = []
a.append(1)
b.append(1)
a.append(2)
```

```expect
3:1 py/repeated-append
```

### case: a comment between the appends

```py
a = []
a.append(1)
# why
a.append(2)
```

```expect
2:1 py/repeated-append
```

### case: inside a function

```py
def f():
    out = []
    out.append(1)
    out.append(2)
    out.append(3)
    return out
```

```expect
3:5 py/repeated-append
```

```fixed
def f():
    out = []
    out.extend((1, 2, 3))
    return out
```
