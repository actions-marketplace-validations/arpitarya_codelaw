# py/undocumented-public-method — cases

### case: overloads, private and nested functions are not reported

```py
class Store:
    @overload
    def get(self, a: int) -> int: ...

    def _load(self):
        def helper():
            pass
```

```expect
(none)
```
