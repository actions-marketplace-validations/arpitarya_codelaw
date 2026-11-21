# py/future-rewritable-type-annotation — cases

### case: a typing collection on a 3.8 target

```py python=3.8
from typing import List, Dict


def f(x: List[int]) -> Dict[str, int]:
    return {}
```

```expect
4:10 py/future-rewritable-type-annotation
4:24 py/future-rewritable-type-annotation
```

```fixed
from __future__ import annotations

from typing import List, Dict


def f(x: List[int]) -> Dict[str, int]:
    return {}
```

### case: a typing collection on a 3.9 target

```py python=3.9
from typing import List


def f(x: List[int]) -> None: ...
```

```expect
(none)
```

### case: Optional and Union on a 3.9 target

```py python=3.9
import typing


def f(x: typing.Optional[int], y: typing.Union[int, str]) -> None: ...
```

```expect
4:10 py/future-rewritable-type-annotation
4:35 py/future-rewritable-type-annotation
```

```fixed
from __future__ import annotations

import typing


def f(x: typing.Optional[int], y: typing.Union[int, str]) -> None: ...
```

### case: Optional on a 3.10 target

```py python=3.10
from typing import Optional


def f(x: Optional[int]) -> None: ...
```

```expect
(none)
```

### case: no target

```py
from typing import Optional


def f(x: Optional[int]) -> None: ...
```

```expect
(none)
```

### case: the import is already there

```py python=3.8
from __future__ import annotations

from typing import List


def f(x: List[int]) -> None: ...
```

```expect
(none)
```

### case: a name that is not typing's

```py python=3.8
from mylib import List


def f(x: List[int]) -> None: ...
```

```expect
(none)
```

### case: a function-local annotation is not evaluated

```py python=3.8
from typing import List


def f() -> None:
    x: List[int] = []
```

```expect
(none)
```

### case: a quoted annotation is not evaluated

```py python=3.8
from typing import List


def f(x: "List[int]") -> None: ...
```

```expect
(none)
```
