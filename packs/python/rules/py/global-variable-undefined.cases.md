# py/global-variable-undefined — cases

### case: defined by assignment, import, def, or not assigned after the global

```py
import sys
COUNT = 0

def helper():
    pass

def a():
    global COUNT, sys, helper
    COUNT = 1
    sys = None
    helper = None

def b():
    global READ_ONLY
    return READ_ONLY
```

```expect
(none)
```

### case: an augmented assignment counts as assigning

```py
def c():
    global total
    total += 1
```

```expect
2:5 py/global-variable-undefined
```
