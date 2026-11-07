# py/dunder-function-name — cases

### case: methods and module __getattr__ are not reported

```py
def __getattr__(name):
    return name

class A:
    def __init__(self):
        pass
```

```expect
(none)
```
