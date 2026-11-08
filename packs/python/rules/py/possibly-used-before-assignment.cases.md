# py/possibly-used-before-assignment — cases

### case: an if without else

```py
def f(a):
    if a:
        x = 1
    return x
```

```expect
4:12 py/possibly-used-before-assignment
```

### case: an elif chain without else

```py
def f(a, b):
    if a:
        x = 1
    elif b:
        x = 2
    return x
```

```expect
6:12 py/possibly-used-before-assignment
```

### case: an else that raises

```py
def f(a):
    if a:
        x = 1
    else:
        raise ValueError
    return x
```

```expect
(none)
```

### case: an if inside a loop, read after the loop

```py
def f(items):
    for i in items:
        if i:
            found = i
    return found
```

```expect
5:12 py/possibly-used-before-assignment
```

### case: a loop alone is not reported

```py
def f(items):
    for i in items:
        found = i
    return found
```

```expect
(none)
```

### case: the same condition guards the read

```py
def f(a):
    if a:
        x = 1
    if a:
        print(x)
```

```expect
(none)
```

### case: a different condition does not

```py
def f(a, b):
    if a:
        x = 1
    if b:
        print(x)
```

```expect
5:15 py/possibly-used-before-assignment
```

### case: every branch binds it

```py
def f(a):
    if a:
        x = 1
    elif not a:
        x = 2
    else:
        x = 3
    return x
```

```expect
(none)
```

### case: a walrus in the true branch of an `and`

```py
def f(d, k):
    if d and (v := d.get(k)):
        return v
```

```expect
(none)
```

### case: module level

```py
import sys

if len(sys.argv) > 1:
    name = sys.argv[1]
print(name)
```

```expect
5:7 py/possibly-used-before-assignment
```
