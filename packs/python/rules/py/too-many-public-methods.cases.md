# py/too-many-public-methods — cases

### case: a setter is not counted

```py options={"max":1}
class A:
    @property
    def x(self):
        pass

    @x.setter
    def x(self, v):
        pass
```

```expect
(none)
```
