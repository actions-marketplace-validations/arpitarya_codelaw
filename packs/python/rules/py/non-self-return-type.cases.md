# py/non-self-return-type — cases

### case: an in-place operator with an annotated self

```py
class Vec:
    def __iadd__(self: Vec, other: Vec) -> Vec:
        return self
```

```expect
2:5 py/non-self-return-type
```

```fixed
from typing import Self

class Vec:
    def __iadd__(self, other: Vec) -> Self:
        return self
```

### case: a final class, a metaclass, another return and an abstract method are fine

```py
from abc import abstractmethod
from typing import final


@final
class A:
    def __enter__(self) -> A:
        return self


class Meta(type):
    def __new__(cls) -> Meta:
        return super().__new__(cls)


class B:
    def __enter__(self) -> int:
        return 1

    @abstractmethod
    def __iadd__(self, other) -> B: ...
```

```expect
(none)
```
