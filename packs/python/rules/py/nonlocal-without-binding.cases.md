# py/nonlocal-without-binding — cases

### case: no enclosing function binds it

```py
def f():
    nonlocal x


def g():
    def h():
        nonlocal y
```

```expect
2:14 py/nonlocal-without-binding
7:18 py/nonlocal-without-binding
```

### case: an enclosing function binds it

```py
def f():
    x = 1

    def g():
        nonlocal x
        x = 2
```

```expect
(none)
```

### case: a parameter, a loop variable, an except name and an import bind

```py
def f(a):
    for i in range(3):
        pass
    try:
        pass
    except Exception as e:
        pass
    import os

    def g():
        nonlocal a, i, e, os
```

```expect
(none)
```

### case: a class between the two does not bind

```py
def f():
    def g():
        class C:
            x = 1

            def m(self):
                nonlocal x
```

```expect
7:26 py/nonlocal-without-binding
```

### case: `del` counts as a binding

```py
def f():
    def g():
        nonlocal x

    del x
```

```expect
(none)
```

### case: `__class__` in a method

```py
class A:
    def method(self):
        nonlocal __class__
```

```expect
(none)
```

### case: a name declared global in the enclosing function is not a binding

```py
x = 0


def f():
    global x

    def g():
        nonlocal x
```

```expect
8:18 py/nonlocal-without-binding
```
