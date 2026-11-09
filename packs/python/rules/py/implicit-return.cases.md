# py/implicit-return — cases

### case: an else that returns

```py
def f(a):
    if a:
        return 1
    else:
        return 2
```

```expect
(none)
```

### case: a loop that may finish

```py
def f(items):
    for i in items:
        if i:
            return i
```

```expect
1:1 py/implicit-return
```

```fixed
def f(items):
    for i in items:
        if i:
            return i
    return None
```

### case: while True and sys.exit end the function

```py
import sys


def f(a):
    if a:
        return 1
    sys.exit(1)


def g(a):
    while True:
        if a:
            return 1
```

```expect
(none)
```

### case: a trailing comment stays on its line

```py
def f(a):
    if a:
        return 1  # found
```

```expect
1:1 py/implicit-return
```

```fixed
def f(a):
    if a:
        return 1  # found
    return None
```

### case: a bare return and None only

```py
def f(a):
    if a:
        return
    return None
```

```expect
(none)
```
