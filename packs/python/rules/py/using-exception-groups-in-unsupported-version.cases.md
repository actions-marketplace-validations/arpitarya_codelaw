# py/using-exception-groups-in-unsupported-version — cases

### case: except* on a 3.10 target

```py python=3.10
try:
    run()
except* ValueError:
    pass
except* (KeyError, OSError) as group:
    pass
```

```expect
3:1 py/using-exception-groups-in-unsupported-version
5:1 py/using-exception-groups-in-unsupported-version
```

### case: except* on a 3.11 target

```py python=3.11
try:
    run()
except* ValueError:
    pass
```

```expect
(none)
```

### case: a plain except

```py python=3.9
try:
    run()
except ValueError:
    pass
```

```expect
(none)
```

### case: no target

```py
try:
    run()
except* ValueError:
    pass
```

```expect
(none)
```
