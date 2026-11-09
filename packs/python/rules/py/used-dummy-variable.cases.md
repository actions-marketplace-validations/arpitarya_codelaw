# py/used-dummy-variable — cases

### case: a local that is read

```py
def fun():
    _count = 1
    return _count
```

```expect
2:5 py/used-dummy-variable
```

```fixed
def fun():
    count = 1
    return count
```

### case: a name that shadows a builtin gets a trailing underscore

```py
def fun():
    _list = []
    return _list
```

```expect
2:5 py/used-dummy-variable
```

```fixed
def fun():
    list_ = []
    return list_
```

### case: a taken name is reported without a fix

```py
def fun():
    x = 1
    _x = x
    return _x
```

```expect
3:5 py/used-dummy-variable
```

### case: a name read in a nested function that binds the new name

```py
def foo():
    _v = 42

    def bar():
        v = 43
        print(_v)
```

```expect
2:5 py/used-dummy-variable
```

### case: never read

```py
def fun(arg):
    _unused = arg
```

```expect
(none)
```

### case: `_`, `__`, dunders and parameters

```py
def fun(_p, x):
    _ = 1
    __ = 2
    __dunder__ = 3
    return _, __, __dunder__, _p
```

```expect
(none)
```

### case: global and nonlocal

```py
_x = 1


def fun():
    global _x
    _x = 2
    return _x
```

```expect
(none)
```

### case: type declarations

```py
def f():
    from typing import TypeVar
    _T = TypeVar("_T")
    return _T
```

```expect
(none)
```

### case: a class body and a module

```py
_valid = 1
print(_valid)


class A:
    _v = 1
    print(_v)
```

```expect
(none)
```
