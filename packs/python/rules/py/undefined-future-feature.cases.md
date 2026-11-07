# Cases: py/undefined-future-feature

### case: an unknown feature among others is removed alone

```py
from __future__ import annotations, braces
```

```expect
1:37 py/undefined-future-feature
```

```fixed
from __future__ import annotations
```
