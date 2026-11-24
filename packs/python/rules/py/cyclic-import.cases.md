# py/cyclic-import — cases

### case: an import under TYPE_CHECKING closes no cycle

```py files
# pkg/__init__.py
# pkg/a.py
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pkg.b import B


def make() -> "B": ...
# pkg/b.py
from pkg.a import make


class B: ...
```

```expect
(none)
```

### case: a cycle of three modules is reported at each import that closes it

```py files
# pkg/__init__.py
# pkg/a.py
from .b import f
# pkg/b.py
from .c import g
# pkg/c.py
from .a import h
```

```expect
pkg/a.py:1:1 py/cyclic-import
pkg/b.py:1:1 py/cyclic-import
pkg/c.py:1:1 py/cyclic-import
```

### case: a module of the standard library is not a node

```py files
# app.py
import os
import json

print(os.sep, json)
```

```expect
(none)
```

### case: a one-way import is no cycle

```py files
# pkg/__init__.py
# pkg/a.py
from pkg.b import x
# pkg/b.py
x = 1
```

```expect
(none)
```
