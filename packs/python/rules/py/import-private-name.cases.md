# py/import-private-name — cases

### case: a private name of another package

```py files
# pkg/__init__.py
# pkg/_impl.py
def _helper(): ...
# other/__init__.py
# other/app.py
from pkg import _helper
```

```expect
other/app.py:1:17 py/import-private-name
```

### case: a private module of another package

```py files
# pkg/__init__.py
# pkg/_impl.py
def f(): ...
# other/__init__.py
# other/app.py
from pkg._impl import f
import pkg._impl
```

```expect
other/app.py:1:23 py/import-private-name
other/app.py:2:8 py/import-private-name
```

### case: a package uses its own private names

```py files
# pkg/__init__.py
# pkg/_impl.py
def _helper(): ...
# pkg/app.py
from pkg._impl import _helper
from . import _impl
from ._impl import _helper as h
```

```expect
(none)
```

### case: a dunder is not private

```py files
# app.py
from pkg import __version__
from pkg.__main__ import run
```

```expect
(none)
```

### case: an import that exists for the type checker

```py files
# app.py
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pkg import _Typed
```

```expect
(none)
```
