# py/unnecessary-dict-kwargs — cases

### case: a keyword or a non-identifier key is left alone

```py
f(**{"class": 1})
f(**{"a-b": 1})
```

```expect
(none)
```
