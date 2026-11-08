# py/deprecated-decorator — cases

### case: a name of the same spelling from elsewhere is fine

```py
from mylib import coroutine

@coroutine
def f():
    pass
```

```expect
(none)
```

### case: imported by name

```py
from abc import abstractclassmethod

class A:
    @abstractclassmethod
    def make(cls): ...
```

```expect
4:5 py/deprecated-decorator
```
