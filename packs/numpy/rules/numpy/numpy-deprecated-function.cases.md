# numpy/numpy-deprecated-function — cases

### case: each alias is renamed

```py
import numpy as np
a = np.round_(x)
b = np.alltrue(x)
c = np.cumproduct(x)
```

```expect
2:5 numpy/numpy-deprecated-function
3:5 numpy/numpy-deprecated-function
4:5 numpy/numpy-deprecated-function
```

```fixed
import numpy as np
a = np.round(x)
b = np.all(x)
c = np.cumprod(x)
```
