# py/too-few-public-methods — cases

### case: a dataclass is fine

```py
@dataclass
class Point:
    x: int
```

```expect
(none)
```

### case: a class with a base is not read

```py
class Error(Exception):
    pass
```

```expect
(none)
```
