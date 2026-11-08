# py/too-many-instance-attributes — cases

### case: a class method's attributes are not counted

```py options={"max":1}
class Job:
    @classmethod
    def make(cls):
        cls.a = 1
        cls.b = 2
```

```expect
(none)
```
