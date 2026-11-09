# py/unaliased-collections-abc-set-import — cases

### case: already aliased

```py
from collections.abc import Set as AbstractSet
```

```expect
(none)
```

### case: the typing module's Set is not this import

```py
from typing import Set
```

```expect
(none)
```
