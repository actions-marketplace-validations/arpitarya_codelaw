# py/unused-wildcard-import — cases

### case: every name is read

```py files
# pkg/__init__.py
# pkg/consts.py
A = 1
B = 2
# pkg/app.py
from pkg.consts import *

print(A, B)
```

```expect
(none)
```

### case: a literal __all__ is what the wildcard brings

```py files
# pkg/__init__.py
# pkg/listed.py
__all__ = ["one", "two"]


def one(): ...
def two(): ...
def three(): ...
# pkg/app.py
from pkg.listed import *

print(one)
```

```expect
pkg/app.py:1:1 py/unused-wildcard-import
```

### case: a name quoted in an annotation is read

```py files
# pkg/__init__.py
# pkg/types.py
class Alpha: ...
# pkg/app.py
from pkg.types import *

x: "Alpha" = None
```

```expect
(none)
```

### case: a module of the standard library is not judged

```py files
# app.py
from os.path import *
```

```expect
(none)
```

### case: an __init__ re-exports on purpose

```py files
# pkg/__init__.py
from pkg.consts import *
# pkg/consts.py
A = 1
```

```expect
(none)
```
