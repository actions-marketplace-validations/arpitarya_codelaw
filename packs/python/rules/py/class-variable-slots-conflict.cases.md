# py/class-variable-slots-conflict — cases

### case: an annotation without a value is fine

```py
class Style:
    __slots__ = ["_color"]
    _color: int
```

```expect
(none)
```
