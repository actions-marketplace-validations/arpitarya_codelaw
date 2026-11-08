# py/generator-return-from-iter-method — cases

### case: __aiter__ in a stub

```py path=api.pyi
from typing import AsyncGenerator


class Rows:
    def __aiter__(self) -> AsyncGenerator[int, None]: ...
```

```expect
5:5 py/generator-return-from-iter-method
```

```fixed
from typing import AsyncGenerator
from collections.abc import AsyncIterator


class Rows:
    def __aiter__(self) -> AsyncIterator[int]: ...
```

### case: a send type, more parameters, or a body that computes are fine

```py
from typing import Generator


class A:
    def __iter__(self) -> Generator[int, str, None]:
        yield 1


class B:
    def __iter__(self, step) -> Generator[int, None, None]:
        yield step


class C:
    def __iter__(self) -> Generator[int, None, None]:
        total = 0
        yield total
```

```expect
(none)
```
