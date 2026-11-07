# py/unnecessary-dunder-call — cases

### case: inside a dunder method, on super(), with keywords, or a dunder with no builtin

```py
class A:
    def __eq__(self, other):
        return self.x.__eq__(other.x)

    def run(self):
        super().__init__()
        self.__init__()
        d.__getitem__("k")
        v.__round__(ndigits=2)
```

```expect
(none)
```

### case: an operator, membership, and an operator inside a larger expression

```py
total = price.__add__(tax)
found = names.__contains__(n)
twice = 2 * a.__add__(3)
```

```expect
1:9 py/unnecessary-dunder-call
2:9 py/unnecessary-dunder-call
3:13 py/unnecessary-dunder-call
```
