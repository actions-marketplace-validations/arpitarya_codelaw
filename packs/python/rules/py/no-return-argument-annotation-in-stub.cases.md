# py/no-return-argument-annotation-in-stub — cases

### case: star parameters and the qualified name

```py path=api.pyi
import typing

def f(*args: typing.NoReturn, **kw: typing.NoReturn) -> None: ...
```

```expect
3:14 py/no-return-argument-annotation-in-stub
3:37 py/no-return-argument-annotation-in-stub
```

```fixed
import typing

def f(*args: typing.Never, **kw: typing.Never) -> None: ...
```

### case: a return annotation and a default are fine where the return is NoReturn

```py path=api.pyi
from typing import NoReturn

def fail() -> NoReturn: ...
def g(x: int = 1) -> None: ...
```

```expect
(none)
```

### case: a parameter with a default

```py path=api.pyi
from typing import NoReturn

def g(x: NoReturn = ...) -> None: ...
```

```expect
3:10 py/no-return-argument-annotation-in-stub
```
