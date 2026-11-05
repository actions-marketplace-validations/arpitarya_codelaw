# pandas/pandas-use-of-dot-read-table — cases

### case: keyword arguments are kept

```py
import pandas as pd
df = pd.read_table("a.tsv", header=None)
```

```expect
2:6 pandas/pandas-use-of-dot-read-table
```

```fixed
import pandas as pd
df = pd.read_csv("a.tsv", header=None, sep='\t')
```

### case: a call with its own separator carries no edit

```py
import pandas as pd
df = pd.read_table("a.csv", sep=",")
```

```expect
2:6 pandas/pandas-use-of-dot-read-table
```
