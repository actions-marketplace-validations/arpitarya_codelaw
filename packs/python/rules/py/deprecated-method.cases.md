# py/deprecated-method — cases

### case: a call at the target that deprecates it

```py python=3.12
import datetime

now = datetime.datetime.utcnow()
```

```expect
3:7 py/deprecated-method
```

### case: target before the deprecation

```py python=3.11
import datetime

now = datetime.datetime.utcnow()
```

```expect
(none)
```

### case: no target

```py
import datetime

now = datetime.datetime.utcnow()
```

```expect
(none)
```

### case: the class imported by name

```py python=3.12
from datetime import datetime

stamp = datetime.utcfromtimestamp(0)
```

```expect
3:9 py/deprecated-method
```

### case: a function of a module

```py python=3.5
import inspect

inspect.getcallargs(f)
```

```expect
3:1 py/deprecated-method
```

### case: a class built on the spot

```py python=3.10
import threading

name = threading.Thread(target=f).getName()
```

```expect
3:8 py/deprecated-method
```

### case: a method of a variable is not followed

```py python=3.12
import threading

t = threading.Thread(target=f)
t.getName()
```

```expect
(none)
```

### case: the replacement is fine

```py python=3.12
import datetime

datetime.datetime.now(datetime.timezone.utc)
```

```expect
(none)
```

### case: a name that is only referred to

```py python=3.12
import datetime

f = datetime.datetime.utcnow
```

```expect
(none)
```
