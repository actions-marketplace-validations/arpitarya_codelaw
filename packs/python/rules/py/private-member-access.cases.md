# py/private-member-access — cases

### case: self, super and dunders are not reported

```py
class A(B):
    def f(self):
        self._x = 1
        super()._setup()
        return other.__class__, rec._asdict()
```

```expect
(none)
```
