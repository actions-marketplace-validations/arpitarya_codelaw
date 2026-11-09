# py/builtin-attribute-shadowing — cases

### case: a read in a default value, a decorator and an assignment

```py
class C:
    id = 1
    bin = 2
    foo = [bin]

    def f(self, x=[id]):
        return x
```

```expect
4:12 py/builtin-attribute-shadowing
6:20 py/builtin-attribute-shadowing
```

### case: a method read in an annotation

```py
class C:
    def str(self):
        pass

    def name(self) -> str:
        pass
```

```expect
5:23 py/builtin-attribute-shadowing
```

### case: an attribute that is never read in the class body

```py
class C:
    id = 1

    def __init__(self):
        self.dir = '.'

    def method(self):
        return id
```

```expect
(none)
```

### case: a read before the binding is the builtin

```py
class C:
    x = id
    id = 1
```

```expect
(none)
```

### case: an overridden method

```py
from typing import override


class C(Base):
    @override
    def list(self):
        pass

    x = list
```

```expect
(none)
```

### case: future annotations

```py
from __future__ import annotations


class C:
    def str(self):
        pass

    def name(self) -> str:
        pass
```

```expect
(none)
```
