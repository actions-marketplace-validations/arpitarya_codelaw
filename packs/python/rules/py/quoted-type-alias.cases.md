# py/quoted-type-alias — cases

### case: a name the file imports under TYPE_CHECKING keeps its quotes

```py
from typing import TYPE_CHECKING, TypeAlias

if TYPE_CHECKING:
    from foo import Foo

A: TypeAlias = "Foo"
```

```expect
(none)
```

### case: a name imported at run time

```py
from typing import TypeAlias

from foo import Foo

A: TypeAlias = "Foo"
```

```expect
5:16 py/quoted-type-alias
```

### case: a subscript outside a TYPE_CHECKING block may not exist at run time

```py
from typing import TypeAlias

from foo import Foo

A: TypeAlias = "Foo[str]"
B: TypeAlias = "Foo.bar"
```

```expect
(none)
```

### case: a string inside a subscript

```py
from typing import TypeAlias

A: TypeAlias = list["int"]
```

```expect
3:21 py/quoted-type-alias
```

### case: Literal and Annotated metadata are not forward references

```py
from typing import Annotated, Literal, TypeAlias

A: TypeAlias = Literal["x", "y"]
B: TypeAlias = Annotated[int, "meta"]
```

```expect
(none)
```

### case: a name that is bound after the alias

```py
from typing import TypeAlias

A: TypeAlias = "Later"


class Later:
    pass
```

```expect
(none)
```

### case: an alias that names itself

```py
from typing import TypeAlias

A: TypeAlias = "A | None"
```

```expect
(none)
```

### case: a TYPE_CHECKING binding with a run-time twin in the else branch

```py
from typing import TYPE_CHECKING, TypeAlias

from foo import Foo

if TYPE_CHECKING:
    Bar: TypeAlias = Foo
else:
    Bar = Foo

A: TypeAlias = "Bar"
```

```expect
10:16 py/quoted-type-alias
```

### case: a type statement is never evaluated early

```py
type A = "Dict"
type B = Literal["x"]
```

```expect
1:10 py/quoted-type-alias
```

### case: a multi-line string gets parentheses

```py
from typing import TypeAlias

A: TypeAlias = """int
| None"""
```

```expect
3:16 py/quoted-type-alias
```

```fixed
from typing import TypeAlias

A: TypeAlias = (int
| None)
```

### case: a string that already has parentheses

```py
from typing import TypeAlias

A: TypeAlias = ("""int
| None""")
```

```expect
3:17 py/quoted-type-alias
```

```fixed
from typing import TypeAlias

A: TypeAlias = (int
| None)
```
