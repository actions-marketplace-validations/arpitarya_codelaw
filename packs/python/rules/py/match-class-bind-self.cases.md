# py/match-class-bind-self — cases

### case: a literal, an `as`, a wildcard or another class are fine

```py
match v:
    case int(2):
        pass
    case int() as n:
        pass
    case int(_):
        pass
    case Point(x):
        pass
    case tuple((a, b)):
        pass
```

```expect
(none)
```

### case: a nested pattern

```py
match v:
    case [str(s), *rest]:
        pass
```

```expect
2:11 py/match-class-bind-self
```
