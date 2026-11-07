# py/blocking-path-method-in-async-function — cases

### case: path arithmetic, a sync function, open, a nested def

```py
import os
from pathlib import Path

async def a(p):
    os.path.join(p, "x")
    Path(p).with_suffix(".txt")
    Path(p).open()

def b(p):
    return os.path.exists(p)

async def c(p):
    def inner():
        return os.path.exists(p)
    return inner
```

```expect
(none)
```

### case: a pathlib method on a constructed path

```py
import pathlib

async def d(p):
    return pathlib.Path(p).read_text()
```

```expect
4:12 py/blocking-path-method-in-async-function
```
