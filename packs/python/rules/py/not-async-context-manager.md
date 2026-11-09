---
id: py/not-async-context-manager
title: "`async with` takes an async context manager"
languages: [python]
fix: manual
fix_reason: "which object to enter, or whether to use `with`, is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/not-async-context-manager]
since: 0.5.0
---

`async with x:` where `x` is a literal, a call of a builtin such as `slice(None)`, a call of a function of this file decorated with `contextlib.contextmanager` (a synchronous generator manager), or an instance of a class of this file whose chain (every base in the file) does not define both `__aenter__` and `__aexit__`. A class with a base the file imports, a class with a metaclass, a class whose name ends in `Mixin`, and any value the file does not state are not read.

**Why:** Python raises `TypeError: 'X' object does not support the asynchronous context manager protocol` when the block is entered.

```codelaw
native: py/not-async-context-manager
```

## How to fix

1. Use `with` for a synchronous manager, or
2. use `contextlib.asynccontextmanager`, or give the class `__aenter__` and `__aexit__`.

## Examples

### Python

#### Don't

```python
import contextlib


@contextlib.contextmanager
def opened():
    yield


async def main():
    async with opened():
        pass
```

#### Do

```python
import contextlib


@contextlib.asynccontextmanager
async def opened():
    yield


async def main():
    async with opened():
        pass
```
