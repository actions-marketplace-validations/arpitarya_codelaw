# py/fallible-context-manager — cases

### case: a final yield, try/finally, a with block, return after, a plain generator

```py
from contextlib import contextmanager

@contextmanager
def a():
    setup()
    yield

@contextmanager
def b():
    try:
        yield
    finally:
        clean()
    log()

@contextmanager
def c():
    with lock:
        yield
    log()

@contextmanager
def d():
    yield
    return

def e():
    yield 1
    print("done")
```

```expect
(none)
```

### case: an async context manager and a yield in a handler

```py
import contextlib

@contextlib.asynccontextmanager
async def f():
    yield
    await close()

@contextlib.contextmanager
def g():
    try:
        work()
    except E:
        yield
        cleanup()
```

```expect
5:5 py/fallible-context-manager
13:9 py/fallible-context-manager
```
