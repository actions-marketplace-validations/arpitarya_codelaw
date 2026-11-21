# py/relative-beyond-top-level — cases

### case: each level of a nested package

```py files
# top/__init__.py
# top/mid/__init__.py
# top/mid/leaf.py
from . import a
from .. import b
from ... import c
from ...far import d
```

```expect
top/mid/leaf.py:3:1 py/relative-beyond-top-level
top/mid/leaf.py:4:1 py/relative-beyond-top-level
```

### case: a script outside a package

```py files
# script.py
from . import sibling
```

```expect
script.py:1:1 py/relative-beyond-top-level
```

### case: a package's own __init__ may use one level

```py files
# pkg/__init__.py
from . import a
# pkg/a.py
X = 1
```

```expect
(none)
```

### case: one statement with several names is one finding

```py files
# top/__init__.py
# top/leaf.py
from .. import a, b, c
```

```expect
top/leaf.py:1:1 py/relative-beyond-top-level
```
