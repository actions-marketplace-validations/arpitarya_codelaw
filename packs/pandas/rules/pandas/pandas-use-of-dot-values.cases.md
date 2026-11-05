# pandas/pandas-use-of-dot-values — cases

### case: without pandas imported nothing is reported

```python
arr = d.values
```

```expect

```

### case: a values() call is a dict method

```python
import pandas as pd
for v in d.values():
    pass
```

```expect

```
