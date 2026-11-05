# pandas/pandas-use-of-inplace-argument — cases

### case: other arguments are kept

```py
import pandas as pd
df.drop(columns=["a"], inplace=True)
```

```expect
2:24 pandas/pandas-use-of-inplace-argument
```

```fixed
import pandas as pd
df = df.drop(columns=["a"])
```

### case: a receiver that is not a plain name carries no edit

```py
import pandas as pd
frame["a"].fillna(0, inplace=True)
```

```expect
2:22 pandas/pandas-use-of-inplace-argument
```
