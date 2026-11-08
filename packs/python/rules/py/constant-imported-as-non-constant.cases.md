# py/constant-imported-as-non-constant — cases

### case: a single capital letter is a constant too

```py
from m import X as x
```

```expect
1:15 py/constant-imported-as-non-constant
```
