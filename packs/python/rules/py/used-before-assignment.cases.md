# py/used-before-assignment — cases

### case: a read before the only assignment

```py
def f():
    print(x)
    x = 1
```

```expect
2:11 py/used-before-assignment
```

### case: a module name used in its own assignment

```py
MSG = 'hello %s' % MSG
```

```expect
1:20 py/used-before-assignment
```

### case: a function called before its definition in the same function

```py
def outer():
    inner()

    def inner():
        pass
```

```expect
2:5 py/used-before-assignment
```

### case: an import used before it is imported

```py
import time


def f():
    print(time.time())
    import time
```

```expect
5:11 py/used-before-assignment
```

### case: assigned in the try body, read in the handler

```py
def f(text):
    try:
        n = int(text)
    except ValueError:
        print(n)
```

```expect
5:15 py/used-before-assignment
```

### case: assigned first in the try body from a literal

```py
def f(text):
    try:
        n = 0
        n = int(text)
    except ValueError:
        print(n)
```

```expect
(none)
```

### case: assigned in the handler before the read

```py
def f(text):
    try:
        n = int(text)
    except ValueError:
        n = 0
        print(n)
```

```expect
(none)
```

### case: assigned on every branch

```py
def f(a):
    if a:
        x = 1
    else:
        x = 2
    return x
```

```expect
(none)
```

### case: a branch that cannot fall through

```py
import sys


def f(a):
    if a:
        sys.exit(1)
    else:
        x = 1
    return x
```

```expect
(none)
```

### case: a function annotated NoReturn

```py
from typing import NoReturn


def fail() -> NoReturn:
    raise SystemExit


def f(a):
    if a:
        x = 1
    else:
        fail()
    return x
```

```expect
(none)
```

### case: assigned in a loop that runs, read in the loop

```py
def f(items):
    for i in items:
        if i:
            print(prev)
        prev = i
```

```expect
(none)
```

### case: a parameter, a global declaration and a nonlocal

```py
def f(a):
    global g
    print(a, g)
    g = 1
```

```expect
(none)
```

### case: a name only an enclosing scope binds

```py
def f():
    print(os)


import os
```

```expect
(none)
```

### case: an annotation alone does not bind

```py
def f():
    x: int
    print(x)
```

```expect
3:11 py/used-before-assignment
```

### case: a walrus binds where it is evaluated

```py
def f(items):
    if (n := len(items)) > 1:
        return n
    return n
```

```expect
(none)
```

### case: a chained assignment

```py
a = b = 0
print(a, b)
```

```expect
(none)
```

### case: a builtin read at module level before the module binds it

```py
TimeoutError = TimeoutError
```

```expect
(none)
```
