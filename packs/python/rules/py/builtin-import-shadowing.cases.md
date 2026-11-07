# py/builtin-import-shadowing — cases

### case: an alias

```py
import json as input
```

```expect
1:16 py/builtin-import-shadowing
```

### case: a module name

```py
from x import y as z
```

```expect
(none)
```
