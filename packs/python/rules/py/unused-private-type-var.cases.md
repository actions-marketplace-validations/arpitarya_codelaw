# py/unused-private-type-var — cases

### case: a use in a quoted annotation

```py
from typing import TypeVar

_T = TypeVar("_T")


def first(items: "list[_T]"):
    return items[0]
```

```expect
(none)
```

### case: a public name is not private

```py
from typing import ParamSpec

P = ParamSpec("P")
```

```expect
(none)
```

### case: a ParamSpec

```py
from typing import ParamSpec

_P = ParamSpec("_P")
```

```expect
3:1 py/unused-private-type-var
```
