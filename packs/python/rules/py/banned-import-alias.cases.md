# py/banned-import-alias — cases

### case: from-import form

```py options={"banned_aliases":{"tensorflow.keras":["k"]}}
from tensorflow import keras as k
```

```expect
1:1 py/banned-import-alias
```

### case: another alias is fine

```py options={"banned_aliases":{"numpy":["nmp"]}}
import numpy as npy
```

```expect
(none)
```
