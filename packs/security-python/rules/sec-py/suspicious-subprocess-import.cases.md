# sec-py/suspicious-subprocess-import — cases

### case: a from-import

```python
from subprocess import run
```

```expect
1:6 sec-py/suspicious-subprocess-import
```

### case: type-checking only

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import subprocess
```

```expect
(none)
```
