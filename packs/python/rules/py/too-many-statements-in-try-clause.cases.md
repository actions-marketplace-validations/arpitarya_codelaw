# py/too-many-statements-in-try-clause — cases

### case: a try with only finally is not read

```py options={"max":1}
try:
    a()
    b()
finally:
    c()
```

```expect
(none)
```
