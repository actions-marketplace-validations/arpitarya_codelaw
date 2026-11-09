# py/typing-only-standard-library-import — cases

### case: one name of two

```py
from __future__ import annotations

from pathlib import Path, PurePath


def size(path: Path) -> int:
    return len(PurePath("a").parts)
```

```expect
(none)
```

### case: an import that is not used

```py
from pathlib import Path
```

```expect
(none)
```
