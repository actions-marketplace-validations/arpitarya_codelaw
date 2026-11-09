# py/unquoted-type-alias — cases

### case: a name imported at run time

```py
from typing import TypeAlias

from foo import Foo

OptFoo: TypeAlias = Foo | None
```

```expect
(none)
```

### case: already quoted

```py
from typing import TYPE_CHECKING, TypeAlias

if TYPE_CHECKING:
    from foo import Foo

OptFoo: TypeAlias = "Foo | None"
```

```expect
(none)
```

### case: a subscript names two

```py
from typing import TYPE_CHECKING, TypeAlias

if TYPE_CHECKING:
    from foo import Foo, Bar

Pair: TypeAlias = dict[Foo, Bar]
```

```expect
6:24 py/unquoted-type-alias
6:29 py/unquoted-type-alias
```
