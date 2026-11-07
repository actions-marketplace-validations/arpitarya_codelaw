# py/confusing-with-statement — cases

### case: a tuple target, a call, or the name first are fine

```py
with a as (x, y), b:
    pass
with a, b as y:
    pass
with a as x, b():
    pass
with a as x, b as y:
    pass
```

```expect
(none)
```

### case: the bare item after the name

```py
with open(p) as f, ctx:
    pass
```

```expect
1:6 py/confusing-with-statement
```
