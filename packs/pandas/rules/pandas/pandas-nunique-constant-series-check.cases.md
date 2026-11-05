# pandas/pandas-nunique-constant-series-check — cases

### case: the negated forms

```py
import pandas as pd
a = df["x"].nunique() != 1
b = s.nunique() > 1
```

```expect
2:5 pandas/pandas-nunique-constant-series-check
3:5 pandas/pandas-nunique-constant-series-check
```

```fixed
import pandas as pd
a = (not (df["x"].iloc[0] == df["x"]).all())
b = (not (s.iloc[0] == s).all())
```
