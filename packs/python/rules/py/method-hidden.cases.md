# py/method-hidden — cases

### case: an attribute with a method's name

```py
class Door:
    def __init__(self):
        self.open = False

    def open(self):
        return True
```

```expect
5:9 py/method-hidden
```

### case: different names

```py
class Door:
    def __init__(self):
        self.is_open = False

    def open(self):
        return True
```

```expect
(none)
```

### case: a property

```py
class Door:
    def __init__(self):
        self.open = False

    @property
    def open(self):
        return True
```

```expect
(none)
```
