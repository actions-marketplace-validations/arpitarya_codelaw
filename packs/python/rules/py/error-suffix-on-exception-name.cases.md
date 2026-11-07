# py/error-suffix-on-exception-name — cases

### case: a base not ending in Exception is not reported

```py
class Missing(KeyError):
    pass
```

```expect
(none)
```

### case: a dotted base

```py
class Bad(errors.BaseException):
    pass
```

```expect
1:1 py/error-suffix-on-exception-name
```
