# py/private-type-parameter — cases

### case: a class

```py
class Box[_T]:
    item: _T
```

```expect
1:11 py/private-type-parameter
```

```fixed
class Box[T]:
    item: T
```

### case: sunder, dunder and public names are fine

```py
class A[_T_, __U__, V]:
    pass
```

```expect
(none)
```

### case: the new name is taken, so the fix is by hand

```py
T = 1


def first[_T](items: list[_T]) -> _T:
    return items[0]
```

```expect
4:11 py/private-type-parameter
```

### case: a TypeVarTuple is renamed by hand

```py
def pack[*_Ts](*args: *_Ts) -> tuple[*_Ts]:
    return args
```

```expect
1:11 py/private-type-parameter
```


### case: a bound

```py
def clamp[_N: int](n: _N) -> _N:
    return n
```

```expect
1:11 py/private-type-parameter
```

```fixed
def clamp[N: int](n: N) -> N:
    return n
```
