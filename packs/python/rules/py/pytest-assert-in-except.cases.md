# py/pytest-assert-in-except — cases

### case: an assert not about the exception is not reported

```py
try:
    f()
except ValueError as e:
    assert ok
```

```expect
(none)
```
