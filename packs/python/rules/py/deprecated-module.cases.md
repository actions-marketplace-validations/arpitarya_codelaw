# py/deprecated-module — cases

### case: import at the target that deprecates it

```py python=3.6
import asyncore
import os
```

```expect
1:8 py/deprecated-module
```

### case: target before the deprecation

```py python=3.5
import asyncore
```

```expect
(none)
```

### case: no target

```py
import asyncore
```

```expect
(none)
```

### case: a submodule is its module's

```py python=3.10
from distutils.core import setup
import distutils.util
```

```expect
1:6 py/deprecated-module
2:8 py/deprecated-module
```

### case: an alias and several names on one line

```py python=3.12
import os, imp as loader, sys
```

```expect
1:12 py/deprecated-module
```

### case: a module that merely starts with the name

```py python=3.12
import imports
import cgi_utils
from . import imp
```

```expect
(none)
```

### case: a removed module still reports

```py python=3.13
import telnetlib
```

```expect
1:8 py/deprecated-module
```
