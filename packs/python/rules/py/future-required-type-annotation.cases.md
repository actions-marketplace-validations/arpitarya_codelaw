# py/future-required-type-annotation — cases

### case: a union on a 3.9 target

```py python=3.9
def f(x: int | None) -> str | int:
    return x
```

```expect
1:10 py/future-required-type-annotation
1:25 py/future-required-type-annotation
```

```fixed
from __future__ import annotations

def f(x: int | None) -> str | int:
    return x
```

### case: a union on a 3.10 target

```py python=3.10
def f(x: int | None) -> str:
    return x
```

```expect
(none)
```

### case: a union with no target

```py
def f(x: int | None) -> str:
    return x
```

```expect
(none)
```

### case: a union below 3.7 cannot take the import

```py python=3.6
def f(x: int | None) -> str:
    return x
```

```expect
(none)
```

### case: a collection on a 3.8 target

```py python=3.8
def f(x: list[int], y: "dict[str, int]") -> tuple[int, ...]:
    return x
```

```expect
1:10 py/future-required-type-annotation
1:45 py/future-required-type-annotation
```

```fixed
from __future__ import annotations

def f(x: list[int], y: "dict[str, int]") -> tuple[int, ...]:
    return x
```

### case: a collection on a 3.9 target

```py python=3.9
def f(x: list[int]) -> dict[str, int]:
    return {}
```

```expect
(none)
```

### case: a union of three is one finding

```py python=3.8
def f(x: int | str | None) -> None: ...
```

```expect
1:10 py/future-required-type-annotation
```

```fixed
from __future__ import annotations

def f(x: int | str | None) -> None: ...
```

### case: the import is already there

```py python=3.8
from __future__ import annotations


def f(x: int | None, y: list[int]) -> None: ...
```

```expect
(none)
```

### case: a docstring comes first

```py python=3.8
"""Doc."""

import os


def f(x: int | None) -> None: ...
```

```expect
6:10 py/future-required-type-annotation
```

```fixed
"""Doc."""

from __future__ import annotations

import os


def f(x: int | None) -> None: ...
```

### case: another future import is extended

```py python=3.8
from __future__ import division


def f(x: int | None) -> None: ...
```

```expect
4:10 py/future-required-type-annotation
```

```fixed
from __future__ import division
from __future__ import annotations


def f(x: int | None) -> None: ...
```

### case: a variable annotation inside a function is not evaluated

```py python=3.8
def f() -> None:
    x: int | None = None
    y: list[int] = []
```

```expect
(none)
```

### case: a module variable annotation runs

```py python=3.8
x: int | None = None
```

```expect
1:4 py/future-required-type-annotation
```

```fixed
from __future__ import annotations

x: int | None = None
```

### case: a stub is left alone

```py path=stubs/a.pyi python=3.8
def f(x: int | None) -> None: ...
```

```expect
(none)
```
