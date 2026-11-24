# py/import-error — cases

### case: a relative import of a missing module

```py files
# pkg/__init__.py
# pkg/app.py
from .gone import x
```

```expect
pkg/app.py:1:1 py/import-error
```

### case: a guarded import is left alone

```py files
# pkg/__init__.py
# pkg/app.py
try:
    from pkg.optional import fast
except ImportError:
    fast = None
```

```expect
(none)
```

### case: an installed package is not judged

```py files
# app.py
import requests
import requests.adapters
import yaml.constructor
```

```expect
(none)
```

### case: a folder without __init__ may be extended

```py files
# ns/inner.py
X = 1
# app.py
import ns.other
```

```expect
(none)
```

### case: a module under a src layout

```py files
# pyproject.toml
[project]
name = "x"
# src/lib/__init__.py
# src/lib/core.py
A = 1
# tests/test_core.py
import lib.core
import lib.nothing
```

```expect
tests/test_core.py:2:1 py/import-error
```

### case: a file that changes sys.path is not judged

```py files
# pkg/__init__.py
# pkg/tool.py
import sys

sys.path.insert(0, "vendor")
import pkg.vendored
```

```expect
(none)
```
