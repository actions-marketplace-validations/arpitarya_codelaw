# py/unused-private-protocol — cases

### case: a protocol with a type argument

```py
from typing import Protocol, TypeVar

_T = TypeVar("_T")


class _Box(Protocol[_T]):
    def get(self) -> _T: ...
```

```expect
6:7 py/unused-private-protocol
```

### case: a plain private class is not a protocol

```py
class _Helper:
    pass
```

```expect
(none)
```
