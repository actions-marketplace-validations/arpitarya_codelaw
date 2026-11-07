### case: an existing import is used, and != is negated

```python
import math

ok = total != 0.3
```

```expect
3:6 py/float-equality-comparison
```

```fixed
import math

ok = not math.isclose(total, 0.3)
```
