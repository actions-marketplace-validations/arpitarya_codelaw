# py/inconsistent-mro — cases

### case: a base before its subclass

```py
class A: ...


class B(A): ...


class C(A, B): ...
```

```expect
7:7 py/inconsistent-mro
```

### case: the subclass first

```py
class A: ...


class B(A): ...


class C(B, A): ...
```

```expect
(none)
```

### case: independent bases and imported ones

```py
from m import X


class A: ...


class B: ...


class C(A, B, X): ...
```

```expect
(none)
```

### case: a diamond

```py
class A: ...


class B(A): ...


class C(A): ...


class D(B, C): ...
```

```expect
(none)
```
