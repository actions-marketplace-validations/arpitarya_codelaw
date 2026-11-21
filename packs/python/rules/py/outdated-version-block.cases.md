# py/outdated-version-block — cases

### case: always true, with an else

```py python=3.9
import sys

if sys.version_info >= (3, 8):
    a = 1
    b = 2
else:
    a = 0
```

```expect
3:1 py/outdated-version-block
```

```fixed
import sys

a = 1
b = 2
```

### case: always false, with an else

```py python=3.9
import sys

if sys.version_info < (3, 8):
    a = 0
else:
    a = 1
    b = 2
```

```expect
3:1 py/outdated-version-block
```

```fixed
import sys

a = 1
b = 2
```

### case: always false, no else

```py python=3.9
import sys

if sys.version_info < (3, 8):
    import old

x = 1
```

```expect
3:1 py/outdated-version-block
```

```fixed
import sys


x = 1
```

### case: nested in a function

```py python=3.9
import sys


def f():
    if sys.version_info >= (3, 9):
        return 1
    return 2
```

```expect
5:5 py/outdated-version-block
```

```fixed
import sys


def f():
    return 1
    return 2
```

### case: the target leaves it open

```py python=3.9
import sys

if sys.version_info >= (3, 10):
    a = 1
else:
    a = 0

if sys.version_info < (3, 10):
    a = 0
```

```expect
(none)
```

### case: a one-number tuple and the major version

```py python=3.9
import sys

if sys.version_info >= (3,):
    a = 1

if sys.version_info[0] == 3:
    b = 1
else:
    b = 0

if sys.version_info[0] < 3:
    c = 1
```

```expect
3:1 py/outdated-version-block
6:1 py/outdated-version-block
11:1 py/outdated-version-block
```

```fixed
import sys

a = 1

b = 1

```

### case: exactly the target

```py python=3.9
import sys

if sys.version_info >= (3, 9):
    a = 1
```

```expect
3:1 py/outdated-version-block
```

```fixed
import sys

a = 1
```

### case: an elif chain is reported without an edit

```py python=3.9
import sys

if flag:
    a = 1
elif sys.version_info >= (3, 8):
    a = 2
```

```expect
5:1 py/outdated-version-block
```

### case: a version imported from sys

```py python=3.9
from sys import version_info

if version_info >= (3, 8):
    a = 1
```

```expect
3:1 py/outdated-version-block
```

```fixed
from sys import version_info

a = 1
```

### case: no target

```py
import sys

if sys.version_info >= (3, 8):
    a = 1
```

```expect
(none)
```
