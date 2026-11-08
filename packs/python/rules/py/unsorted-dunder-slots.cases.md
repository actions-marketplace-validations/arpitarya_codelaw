# py/unsorted-dunder-slots — cases

### case: a list across lines

```py
class Point:
    __slots__ = [
        "z",
        "a",
    ]
```

```expect
2:17 py/unsorted-dunder-slots
```

```fixed
class Point:
    __slots__ = [
        "a",
        "z",
    ]
```

### case: outside a class, and one name, are fine

```py
__slots__ = ("b", "a")


class A:
    __slots__ = ("only",)
```

```expect
(none)
```
