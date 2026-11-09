# py/typing-only-first-party-import — cases

### case: a run-time use

```py
from __future__ import annotations

from .models import User


def make(user: User) -> User:
    return User()
```

```expect
(none)
```

### case: no future import: the annotation runs

```py
from .models import User


def greet(user: User) -> str:
    return "hi"
```

```expect
(none)
```
