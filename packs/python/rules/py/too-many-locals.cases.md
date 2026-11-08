# py/too-many-locals — cases

### case: a loop variable does not count

```py options={"max":1}
def f(xs):
    a = 1
    for b in xs:
        pass
    return a
```

```expect
(none)
```
