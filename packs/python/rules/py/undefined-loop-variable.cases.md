# py/undefined-loop-variable — cases

### case: a read after a loop over an unknown iterable

```py
def last(items):
    for item in items:
        pass
    return item
```

```expect
4:12 py/undefined-loop-variable
```

### case: a loop that breaks

```py
def find(items):
    for item in items:
        if item:
            break
    return item
```

```expect
5:12 py/undefined-loop-variable
```

### case: a non-empty literal

```py
def f():
    for v in [1, 2, 3]:
        pass
    for w in range(1, 3):
        pass
    return v, w
```

```expect
(none)
```

### case: range shadowed

```py
def f():
    def range(n):
        yield n

    for v in range(3):
        pass
    return v
```

```expect
7:12 py/undefined-loop-variable
```

### case: an else that binds it

```py
def f(items):
    for v in items:
        if v:
            break
    else:
        v = 0
    return v
```

```expect
(none)
```

### case: an else that returns

```py
def f(items):
    for v in items:
        break
    else:
        return None
    return v
```

```expect
(none)
```

### case: bound before the loop

```py
def f(items):
    v = 0
    for v in items:
        pass
    return v
```

```expect
(none)
```

### case: enumerate over a non-empty range

```py
def f():
    for i, n in enumerate(range(3)):
        pass
    return i, n
```

```expect
(none)
```

### case: a tuple target at module level

```py
pairs = [(1, 2)]
for a, b in pairs:
    pass
print(a)
```

```expect
4:7 py/undefined-loop-variable
```
