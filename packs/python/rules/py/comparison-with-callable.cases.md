# py/comparison-with-callable — cases

### case: a function against a literal

```py
def is_ready():
    return True


if is_ready == True:
    pass
```

```expect
5:4 py/comparison-with-callable
```

### case: != and the function on the right

```py
def level():
    return 1


x = 1 != level
```

```expect
5:5 py/comparison-with-callable
```

### case: called

```py
def is_ready():
    return True


if is_ready() == True:
    pass
```

```expect
(none)
```

### case: two names

```py
def a():
    pass


b = a
x = a == b
```

```expect
(none)
```

### case: a decorated function

```py
import functools


@functools.cache
def f():
    return 1


x = f == 1
```

```expect
(none)
```
