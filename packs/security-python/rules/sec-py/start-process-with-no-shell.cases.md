# sec-py/start-process-with-no-shell — cases

### case: imported by name

```python
from os import execv

execv("/bin/ls", ["ls"])
```

```expect
3:1 sec-py/start-process-with-no-shell
```

### case: os.system is another rule's

```python
import os

os.system("ls")
```

```expect
(none)
```
