# py/missing-return-type-undocumented-public-function — cases

### case: one that returns a value is reported without a fix

```py
def total(xs):
    return sum(xs)
```

```expect
1:1 py/missing-return-type-undocumented-public-function
```
