### case: binary mode reads bytes

```python
from pathlib import Path

with open(path, "rb") as f:
    data = f.read()
```

```expect
3:1 py/read-whole-file
```

```fixed
from pathlib import Path

data = Path(path).read_bytes()
```
