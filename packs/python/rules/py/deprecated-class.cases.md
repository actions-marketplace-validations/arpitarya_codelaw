# py/deprecated-class — cases

### case: from-import of a deprecated class

```py python=3.7
from importlib.abc import ResourceLoader, Loader
```

```expect
1:27 py/deprecated-class
```

### case: the dotted spelling

```py python=3.9
import importlib.abc

class L(importlib.abc.ResourceLoader):
    pass
```

```expect
3:9 py/deprecated-class
```

### case: target before the deprecation

```py python=3.6
from importlib.abc import ResourceLoader
```

```expect
(none)
```

### case: no target

```py
from importlib.abc import ResourceLoader
```

```expect
(none)
```

### case: an aliased import

```py python=3.9
from importlib.abc import ResourceLoader as RL
```

```expect
1:27 py/deprecated-class
```

### case: the same name from another module

```py python=3.12
from mylib.abc import ResourceLoader
```

```expect
(none)
```

### case: a module attribute is the attribute rule's

```py python=3.10
import ssl

ssl.PROTOCOL_TLS
```

```expect
(none)
```

### case: a typing alias that PEP 585 replaced is not this rule's

```py python=3.12
from typing import Dict
```

```expect
(none)
```
