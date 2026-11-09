# py/runtime-import-in-type-checking-block — cases

### case: used only in an annotation, under the future import

```py
from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path


def f(p: Path) -> None: ...
```

```expect
(none)
```
