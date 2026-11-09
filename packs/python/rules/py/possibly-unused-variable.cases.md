# py/possibly-unused-variable — cases

### case: assigned before locals()

```py
def f():
    value = 42
    return locals()
```

```expect
2:5 py/possibly-unused-variable
```

### case: assigned after locals()

```py
def f():
    seen = locals()
    value = 42
    return seen
```

```expect
(none)
```

### case: locals() in a nested function

```py
def f():
    value = 42

    def inner():
        return locals()
    return inner
```

```expect
(none)
```

### case: the variable is read

```py
def f():
    value = 42
    print(value)
    return locals()
```

```expect
(none)
```

### case: a dummy name

```py
def f():
    _x = 1
    unused_y = 2
    return locals()
```

```expect
(none)
```
