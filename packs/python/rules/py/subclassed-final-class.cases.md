# py/subclassed-final-class — cases

### case: a subclass

```py
from typing import final


@final
class Base: ...


class Child(Base): ...
```

```expect
8:7 py/subclassed-final-class
```

### case: a class that is not final

```py
class Base: ...


class Child(Base): ...
```

```expect
(none)
```

### case: the qualified decorator

```py
import typing


@typing.final
class Base: ...


class Child(Base): ...
```

```expect
8:7 py/subclassed-final-class
```
