# py/redundant-typehint-argument — cases

### case: parameters, nested unions and different types are not read

```py
def f(p: int | int) -> None:
    pass
m: dict[int | int, str] = {}
n: int | str = 1
```

```expect
(none)
```

### case: Union and Optional of None

```py
from typing import Optional, Union
a: Union[str, str] = ""
b: Optional[None] = None
```

```expect
2:4 py/redundant-typehint-argument
3:4 py/redundant-typehint-argument
```
