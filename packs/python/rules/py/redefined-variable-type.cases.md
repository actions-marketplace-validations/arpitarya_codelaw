# py/redefined-variable-type — cases

### case: a function

```py
def parse(raw):
    count = 0
    count = "none"
    return count
```

```expect
3:5 py/redefined-variable-type
```

### case: an attribute

```py
class A:
    def __init__(self):
        self.var1 = 2
        self.var1 = 2.
```

```expect
4:9 py/redefined-variable-type
```

### case: int and bool

```py
def f():
    myint = 2
    myint = False
    return myint
```

```expect
3:5 py/redefined-variable-type
```

### case: a ternary whose arms agree

```py
def f(c):
    a_str = False
    a_str = 2.0 if c else 1.0
    return a_str
```

```expect
3:5 py/redefined-variable-type
```

### case: a ternary whose arms differ

```py
def f(c):
    a = False
    a = 2.0 if c else 1
    return a
```

```expect
(none)
```

### case: the module

```py
_OK = True
_OK = "this is ok"
```

```expect
2:1 py/redefined-variable-type
```

### case: a class instance

```py
class MyClass:
    pass


def other():
    instance = MyClass()
    instance = True
    return instance
```

```expect
7:5 py/redefined-variable-type
```

### case: a function's return

```py
def dummy():
    return 2


SOME_FLOAT = 1.
SOME_FLOAT = dummy()
```

```expect
6:1 py/redefined-variable-type
```

### case: None and a list

```py
A_GLOB = None
A_GLOB = [1, 2, 3]
```

```expect
(none)
```

### case: branches of one if

```py
def f(x):
    if x:
        var = "foo"
    else:
        var = True
    if x:
        var2 = "foo"
    elif not x:
        var2 = 2
    return var, var2
```

```expect
(none)
```

### case: after the branches

```py
def f(x):
    if x:
        var = "foo"
    else:
        var = True
    var = 2
    return var
```

```expect
6:5 py/redefined-variable-type
```

### case: twice in one branch

```py
def f(x):
    if x:
        var3 = "foo"
        var3 = 2
    return var3
```

```expect
4:9 py/redefined-variable-type
```

### case: another scope

```py
class A:
    def __init__(self):
        self.var = True

    def other(self):
        self.var = 1

    def nested(self):
        def func():
            var = 1
            var = "baz"
            return var

        var = "x"
        return func
```

```expect
11:13 py/redefined-variable-type
```

### case: a bare underscore

```py
_ = []
_ = object()


def f():
    _ = []
    _ = object()
```

```expect
(none)
```

### case: a tuple target and an annotation

```py
def f():
    (a, b) = (1, 2)
    a = "x"
    c: int = 1
    c = "y"
    return a, b, c
```

```expect
(none)
```
