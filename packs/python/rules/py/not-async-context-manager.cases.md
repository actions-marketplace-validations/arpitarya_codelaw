# py/not-async-context-manager — cases

### case: a literal

```py
async def main():
    async with 42:
        pass
```

```expect
2:16 py/not-async-context-manager
```

### case: a synchronous contextmanager

```py
import contextlib


@contextlib.contextmanager
def opened():
    yield


async def main():
    async with opened():
        pass
```

```expect
10:16 py/not-async-context-manager
```

### case: an asynccontextmanager

```py
import contextlib


@contextlib.asynccontextmanager
async def opened():
    yield


async def main():
    async with opened():
        pass
```

```expect
(none)
```

### case: a synchronous class

```py
class Sync:
    def __enter__(self):
        pass

    def __exit__(self, *args):
        pass


async def main():
    async with Sync():
        pass
```

```expect
10:16 py/not-async-context-manager
```

### case: half of the protocol

```py
class Half:
    async def __aenter__(self):
        pass


async def main():
    async with Half():
        pass
```

```expect
7:16 py/not-async-context-manager
```

### case: the protocol split across a base

```py
class Exit:
    async def __aexit__(self, *args):
        pass


class Both(Exit):
    async def __aenter__(self):
        pass


async def main():
    async with Both():
        pass
```

```expect
(none)
```

### case: a builtin

```py
async def main():
    async with slice(None):
        pass
```

```expect
2:16 py/not-async-context-manager
```

### case: a base the file imports

```py
from lib import Base


class Child(Base):
    pass


async def main():
    async with Child():
        pass
```

```expect
(none)
```

### case: a Mixin

```py
class ManagerMixin:
    pass


async def main():
    async with ManagerMixin():
        pass
```

```expect
(none)
```

### case: a plain with

```py
async def main():
    with 42:
        pass
```

```expect
(none)
```
