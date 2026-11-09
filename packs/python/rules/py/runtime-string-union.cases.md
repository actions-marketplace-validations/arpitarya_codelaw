# py/runtime-string-union — cases

### case: the future import makes annotations strings

```py
from __future__ import annotations

var: "Foo" | None = None


class Foo: ...
```

```expect
(none)
```

### case: a parameter annotation

```py
def f(x: int | "Foo") -> None: ...


class Foo: ...
```

```expect
1:16 py/runtime-string-union
```
