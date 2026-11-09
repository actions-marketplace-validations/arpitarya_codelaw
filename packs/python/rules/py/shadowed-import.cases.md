# py/shadowed-import — cases

### case: a second import of the name

```py
from pathlib import Path
from some_other_lib import CustomPath as Path
```

```expect
2:1 py/shadowed-import
```

### case: a module and an aliased module

```py
from pandas._libs import algos
import pandas.core.algorithms as algos
```

```expect
2:1 py/shadowed-import
```

### case: the same import twice

```py
from pathlib import Path
from pathlib import Path
```

```expect
(none)
```

### case: different names

```py
from sklearn._libs import second as libalgos
import sklearn.core.algorithms as second
```

```expect
(none)
```

### case: import a.b and import a.c

```py
import os.path
import os
```

```expect
(none)
```

### case: try and except

```py
try:
    import json
except ImportError:
    import simplejson as json
```

```expect
(none)
```

### case: a function body

```py
from pathlib import Path


def f():
    from lib import Path
    return Path
```

```expect
(none)
```

### case: in a function after the same name

```py
def f():
    from pathlib import Path
    from lib import Path
    return Path
```

```expect
3:5 py/shadowed-import
```
