# py/invalid-function-name — cases

### case: unittest hooks and overrides are not reported

```py
class T(unittest.TestCase):
    def setUp(self):
        pass

    @override
    def doThing(self):
        pass
```

```expect
(none)
```

### case: a method

```py
class T:
    def getName(self):
        return 1
```

```expect
2:9 py/invalid-function-name
```
