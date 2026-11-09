# py/custom-type-var-for-self — cases

### case: a class method

```py
from typing import TypeVar

_T = TypeVar("_T", bound="A")


class A:
    @classmethod
    def make(cls: type[_T]) -> _T:
        return cls()
```

```expect
8:13 py/custom-type-var-for-self
```

```fixed
from typing import TypeVar, Self

_T = TypeVar("_T", bound="A")


class A:
    @classmethod
    def make(cls) -> Self:
        return cls()
```

### case: a static method is left alone

```py
from typing import TypeVar

_T = TypeVar("_T")


class A:
    @staticmethod
    def f(x: _T) -> _T:
        return x
```

```expect
(none)
```

### case: the variable named in a string: reported, no edit

```py
from typing import TypeVar

_S = TypeVar("_S", bound="A")


class A:
    def f(self: _S) -> "list[_S]":
        return [self]
```

```expect
7:10 py/custom-type-var-for-self
```
