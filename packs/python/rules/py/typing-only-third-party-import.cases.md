# py/typing-only-third-party-import — cases

### case: a quoted annotation

```py
from requests import Session


def close(session: "Session") -> None:
    session.close()
```

```expect
1:22 py/typing-only-third-party-import
```

### case: an existing block takes the import

```py
from __future__ import annotations

from typing import TYPE_CHECKING

from requests import Session

if TYPE_CHECKING:
    from other import Thing


def close(session: Session, t: Thing) -> None:
    session.close()
```

```expect
5:22 py/typing-only-third-party-import
```
