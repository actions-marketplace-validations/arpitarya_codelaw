# py/confusing-consecutive-elif — cases

### case: an inner `else`, a later statement, or a separate `if` are fine

```py
if a:
    if b:
        x()
    elif c:
        y()
    else:
        z()
elif d:
    w()
if a:
    if b:
        x()
    done()
elif d:
    w()
if a:
    if b:
        x()
else:
    w()
```

```expect
(none)
```

### case: after an earlier `elif`

```py
if a:
    x()
elif b:
    if c:
        y()
elif d:
    z()
```

```expect
6:1 py/confusing-consecutive-elif
```
