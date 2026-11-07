# py/non-lowercase-variable-in-function — cases

### case: a TypeVar is not reported

```py
def f():
    T = TypeVar("T")
    return T
```

```expect
(none)
```

### case: module level is not reported

```py
Total = 1
```

```expect
(none)
```
