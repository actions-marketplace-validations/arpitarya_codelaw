# py/contextmanager-generator-missing-cleanup — cases

### case: a manager with code after its yield

```py
from contextlib import contextmanager


@contextmanager
def cm():
    var = 'acquired'
    print('enter')
    yield var
    print('exit')


def gen():
    with cm() as ctx:
        yield ctx * 2
```

```expect
13:5 py/contextmanager-generator-missing-cleanup
```

### case: a manager defined after the generator

```py
def gen():
    with later() as ctx:
        yield ctx * 2


import contextlib


@contextlib.contextmanager
def later():
    var = 'acquired'
    yield var
    print('exit')
```

```expect
2:5 py/contextmanager-generator-missing-cleanup
```

### case: a manager that only handles another exception

```py
from contextlib import contextmanager


@contextmanager
def cm():
    var = 1
    try:
        yield var
    except ValueError:
        pass
    print('exit')


def gen():
    with cm() as ctx:
        yield ctx
```

```expect
15:5 py/contextmanager-generator-missing-cleanup
```

### case: try/finally in the manager

```py
from contextlib import contextmanager


@contextmanager
def cm():
    var = 1
    try:
        yield var
    finally:
        print('exit')


def gen():
    with cm() as ctx:
        yield ctx
```

```expect
(none)
```

### case: a handler for GeneratorExit, a bare except, Exception

```py
from contextlib import contextmanager


@contextmanager
def a():
    try:
        yield 1
    except GeneratorExit:
        pass


@contextmanager
def b():
    try:
        yield 1
    except:
        pass


def gen():
    with a() as x:
        yield x
    with b() as y:
        yield y
```

```expect
(none)
```

### case: a manager with nothing after its yield

```py
from contextlib import contextmanager


@contextmanager
def cm():
    print('enter')
    yield 1


def gen():
    with cm() as ctx:
        yield ctx
```

```expect
(none)
```

### case: a manager that yields nothing

```py
from contextlib import contextmanager


@contextmanager
def cm():
    yield
    print('exit')


def gen():
    with cm() as ctx:
        yield 'x'
```

```expect
(none)
```

### case: no `as` name, or the yield does not use it

```py
from contextlib import contextmanager


@contextmanager
def cm():
    yield 1
    print('exit')


def gen():
    with cm():
        yield 'x'
```

```expect
(none)
```

### case: the generator is itself a context manager

```py
from contextlib import contextmanager


@contextmanager
def cm():
    yield 1
    print('exit')


@contextmanager
def outer():
    with cm() as ctx:
        yield ctx
```

```expect
(none)
```

### case: a class with __enter__

```py
class E:
    def __enter__(self):
        return self

    def __exit__(self, *a):
        pass


def gen():
    with E() as e:
        yield e
```

```expect
(none)
```
