# py/raise-within-try — cases

### case: another type, a bare raise, finally only, and a raise in the handler

```py
try:
    raise ValueError("x")
except TypeError:
    pass
try:
    work()
except ValueError:
    raise
try:
    raise ValueError("x")
finally:
    clean()
try:
    work()
except ValueError:
    raise KeyError("y")
```

```expect
(none)
```

### case: a broad handler, a tuple, and a nested try read on its own

```py
try:
    raise MyError(a)
except Exception:
    pass
try:
    raise KeyError
except (KeyError, IndexError) as e:
    pass
try:
    try:
        raise A()
    except B:
        pass
except A:
    pass
```

```expect
2:5 py/raise-within-try
6:5 py/raise-within-try
```
