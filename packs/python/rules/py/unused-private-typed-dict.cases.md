# py/unused-private-typed-dict — cases

### case: the call form

```py
from typing import TypedDict

_Point = TypedDict("_Point", {"x": int})
```

```expect
3:1 py/unused-private-typed-dict
```
