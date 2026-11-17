# py/no-name-in-module — cases

### case: a submodule is a name of its package

```py files
# pkg/__init__.py
# pkg/sub.py
X = 1
# app.py
from pkg import sub
from pkg import nothing
```

```expect
app.py:2:17 py/no-name-in-module
```

### case: names a module re-exports and binds in other ways

```py files
# pkg/__init__.py
from .inner import helper
# pkg/inner.py
def helper(): ...
# pkg/mod.py
import json

for item in []:
    pass

with open("f") as handle:
    pass

try:
    from fast import speed
except ImportError:
    speed = None
# app.py
from pkg import helper
from pkg.mod import json, item, handle, speed
```

```expect
(none)
```

### case: a module with a module-level __getattr__ is not judged

```py files
# pkg/__init__.py
# pkg/lazy.py
def __getattr__(name):
    return name
# app.py
from pkg.lazy import anything
```

```expect
(none)
```

### case: a name through a star import of the project

```py files
# pkg/__init__.py
# pkg/base.py
def one(): ...
# pkg/wide.py
from pkg.base import *
# app.py
from pkg.wide import one, two
```

```expect
app.py:1:26 py/no-name-in-module
```

### case: the standard library is not judged

```py files
# app.py
from os import path, nothing
from collections import OrderedDict
```

```expect
(none)
```
