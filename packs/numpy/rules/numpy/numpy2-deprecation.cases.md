# numpy/numpy2-deprecation — cases

### case: aliases are renamed

```py
import numpy as np
a = np.Inf
b = np.float_(1)
c = np.row_stack(x)
```

```expect
2:5 numpy/numpy2-deprecation
3:5 numpy/numpy2-deprecation
4:5 numpy/numpy2-deprecation
```

```fixed
import numpy as np
a = np.inf
b = np.float64(1)
c = np.vstack(x)
```

### case: a removed name with no plain alias is reported only

```py
import numpy as np
c = np.in1d(x, y)
```

```expect
2:5 numpy/numpy2-deprecation
```
